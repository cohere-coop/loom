mod child_with_streams;

use std::io;
use std::process::Command;

use futures;
use futures::{Stream, Future};
use tokio_core::reactor::Core;
use tokio_signal;

use super::CommandParams;
use self::child_with_streams::ChildWithStreams;

pub fn run_commands(config: Vec<CommandParams>) -> Result<(), io::Error> {
    match Core::new() {
        Ok(mut unwrapped_core) => {
            let (printer_vec, exit_status_vec): (Vec<_>, Vec<_>) = config.into_iter().map(|command_params| {
                let mut ping = ChildWithStreams::new(&unwrapped_core.handle(), &mut Command::new(command_params.bin).arg(command_params.arg));
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
            let prog = tokio_signal::ctrl_c(&unwrapped_core.handle()).flatten_stream().into_future();

            match unwrapped_core.run(printers.join(exit_statuses)) {
                Ok((_, final_exit_statuses)) => {
                    final_exit_statuses.into_iter().for_each(|exit_status| {
                        match exit_status.code() {
                            Some(code) => println!("child process exited with code: {:?}", code),
                            None       => panic!("no child process exit code obtainable")
                        }
                    }) 
                },
                Err(e)   => panic!("failed to wait for child process exit: {:?}", e)
            }

            match unwrapped_core.run(prog) {
                Ok(_) => Ok(()),
                Err((e, _)) => Err(e) 
            }
        },
        Err(e) => Err(e)
    }
}
