extern crate futures;
extern crate tokio_core;
extern crate tokio_process;
extern crate tokio_io;

use std::process::{Command, Stdio, ExitStatus};
use std::io::{self, Write};

use futures::{Future, Stream};
use tokio_core::reactor::Core;
use tokio_process::{CommandExt, Child};

fn main() {
    ::std::process::exit(match run_core_loop() {
       Ok(_) => 0,
       Err(err) => {
           writeln!(io::stderr(), "error: {:?}", err).unwrap();
           1
       }
    });
}

fn run_core_loop() -> Result<(), (std::io::Error)> {
    match Core::new() {
        Ok(mut unwrapped_core) => {
            run_command_on_core(&mut unwrapped_core, &mut Command::new("ls"));
            run_command_on_core(&mut unwrapped_core, &mut Command::new("ping").arg("google.com"));
            Ok(())
        },
        Err(e) => { Err(e) } 
    }
}

fn run_command_on_core(core: &mut Core, command: &mut Command) {
    let mut ls = Command::new("ping").arg("wecohere.com")
        .stdout(Stdio::piped())
        .spawn_async(&core.handle())
        .expect("failed to spawn child process");

    let ls_stdout = ls.stdout().take().expect("couldn't capture stdout");
    let ls_reader = io::BufReader::new(ls_stdout);
    let ls_lines = tokio_io::io::lines(ls_reader);
    let ls_cycle = ls_lines.for_each(|line| {
        println!("Line: {}", line);
        Ok(())
    });

    let mut ping = Command::new("ping").arg("google.com")
        .stdout(Stdio::piped())
        .spawn_async(&core.handle())
        .expect("failed to spawn child process");

    let ping_stdout = ping.stdout().take().expect("couldn't capture stdout");
    let ping_reader = io::BufReader::new(ping_stdout);
    let ping_lines = tokio_io::io::lines(ping_reader);
    let ping_cycle = ping_lines.for_each(|line| {
        println!("Line: {}", line);
        Ok(())
    });

    match core.run(ping_cycle) {
        Ok(code) => println!("child process stdout finished"),
        Err(e)   => panic!("child process stdout error: {:?}", e)
    }

    match core.run(ping) {
        Ok(code) => println!("child process exited with code: {}", code),
        Err(e)   => panic!("failed to wait for child process exit: {}", e)
    }

    match core.run(ls_cycle) {
        Ok(code) => println!("child process stdout finished"),
        Err(e)   => panic!("child process stdout error: {:?}", e)
    }

    match core.run(ls) {
        Ok(code) => println!("child process exited with code: {}", code),
        Err(e)   => panic!("failed to wait for child process exit: {}", e)
    }
}

// fn stdout_printer(mut child_process: Child) -> Box<Future<Item = (), Error = io::Error>> {
//     let stdout = child_process.stdout().take().expect("couldn't capture stdout");
//     let reader = io::BufReader::new(stdout);
//     let lines = tokio_io::io::lines(reader);
//     let cycle = lines.for_each(|line| {
//         println!("Line: {}", line);
//         Ok(())
//     });

//     Box::new(cycle.join(child_process))
// } 