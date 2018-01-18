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
            run_command_on_core(&mut unwrapped_core, &mut Command::new("ping").arg("google.com"));
            run_command_on_core(&mut unwrapped_core, &mut Command::new("ls"));
            Ok(())
        },
        Err(e) => { Err(e) } 
    }
}

fn run_command_on_core(core: &mut Core, command: &mut Command) {
    let child = command
        .stdout(Stdio::piped())
        .spawn_async(&core.handle())
        .expect("failed to spawn child process");

    match core.run(stdout_printer(child)) {
        Ok(code) => println!("child process exited with code: {}", code),
        Err(e)   => panic!("failed to wait for child process exit: {}", e)
    }
}

fn stdout_printer(mut child_process: Child) -> Box<Future<Item = ExitStatus, Error = io::Error>> {
    let stdout = child_process.stdout().take().expect("couldn't capture stdout");
    let reader = io::BufReader::new(stdout);
    let lines = tokio_io::io::lines(reader);
    let cycle = lines.for_each(|line| {
        println!("Line: {}", line);
        Ok(())
    });
    Box::new(cycle.join(child_process).map(|((), s)| s))
} 