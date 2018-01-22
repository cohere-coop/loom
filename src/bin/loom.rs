extern crate futures;
extern crate tokio_core;
extern crate tokio_process;
extern crate tokio_io;

extern crate loom;

use std::process::{Command};
use std::io::{self, Write};

use futures::Stream;
use tokio_core::reactor::Core;

use loom::ChildWithStreams;

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
            let mut cohere_ping = ChildWithStreams::new(&unwrapped_core.handle(), &mut Command::new("ping").arg("wecohere.com"));
            let mut google_ping = ChildWithStreams::new(&unwrapped_core.handle(), &mut Command::new("ping").arg("google.com"));

            let mut pings = vec![cohere_ping, google_ping];
            let (printer_vec, exit_status_vec): (Vec<_>, Vec<_>) = pings.into_iter().map(|mut ping| {
                let stdout_lines = *ping.stdout_as_lines().take().unwrap();
                let printer = stdout_lines.for_each(|line| {
                    println!("Line: {}", line);
                    Ok(())
                });
                let exit_status = ping.exit_status().take().unwrap();

                (printer, exit_status)
            }).unzip();

            let printers = futures::future::join_all(printer_vec);
            let exit_statuses = futures::future::join_all(exit_status_vec);

            match unwrapped_core.run(printers) {
                Ok(_) => println!("child process stdout finished"),
                Err(e)   => panic!("child process stdout error: {:?}", e)
            }

            match unwrapped_core.run(exit_statuses) {
                Ok(code) => println!("child process exited with code: {:?}", code),
                Err(e)   => panic!("failed to wait for child process exit: {:?}", e)
            }
            Ok(())
        },
        Err(e) => { Err(e) } 
    }
}
