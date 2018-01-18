extern crate futures;
extern crate tokio_core;
extern crate tokio_process;

use std::process::{Command, Output};
use std::io::{self, Write};

use tokio_core::reactor::Core;
use tokio_process::CommandExt;

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
            let output = run_command_on_core(&mut unwrapped_core, &mut Command::new("ls"));
            print_output_details(output);
            Ok(())
        },
        Err(e) => { Err(e) } 
    }
}

fn run_command_on_core(core: &mut Core, command: &mut Command) -> Output {
    let child = command.output_async(&core.handle());
    core.run(child).expect("failed to capture child output")
}

fn print_output_details(output: Output) {
    match output.status.code() {
        Some(code) => println!("process exited with code: {}", code),
        None       => println!("process terminated by signal")
    }
    match String::from_utf8(output.stdout) {
        Ok(output_string) => println!("process output:\n{}", output_string),
        Err(e)            => println!("could not parse process output:\n {:?}", e)
    }
}