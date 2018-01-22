extern crate futures;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_process;

use std::io;
use std::io::BufReader;
use std::process::{Command, Stdio, ExitStatus};

use futures::{Future};
use tokio_core::reactor::Handle;
use tokio_io::io::{lines, Lines};
use tokio_process::{CommandExt, ChildStdout};

pub struct ChildWithStreams {
    exit_status: Option<Box<Future<Item = ExitStatus, Error = io::Error>>>,
    stdout_as_lines: Option<Box<Lines<BufReader<ChildStdout>>>>,
}

impl ChildWithStreams {
    pub fn new(handle: &Handle, command: &mut Command) -> Self {
        let mut child = command
            .stdout(Stdio::piped())
            .spawn_async(&handle)
            .expect("failed to spawn child process");

        let stdout = child.stdout().take().expect("couldn't capture stdout");
        let reader = BufReader::new(stdout);
        let lines = lines(reader);

        ChildWithStreams {
            exit_status: Some(Box::new(child)),
            stdout_as_lines: Some(Box::new(lines)),
        }
    }

    pub fn exit_status(&mut self) -> &mut Option<Box<Future<Item = ExitStatus, Error = io::Error>>> {
        &mut self.exit_status
    }

    pub fn stdout_as_lines(&mut self) -> &mut Option<Box<Lines<BufReader<ChildStdout>>>> {
        &mut self.stdout_as_lines
    }
}