mod child_with_streams;

use std::io;
use std::process::Command;

use futures;
use futures::Stream;
use tokio_core::reactor::Core;

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
        Err(e) => Err(e)
    }
}
