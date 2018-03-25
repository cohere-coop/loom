use std::io;
use std::io::BufReader;
use std::process::{Command, Stdio, ExitStatus};
use futures::stream::{repeat, Zip, Repeat};
use futures::{Future, Stream};
use tokio_core::reactor::Handle;
use tokio_io::io::{lines, Lines};
use tokio_process::{CommandExt, ChildStdout};

use super::super::CommandParams;

pub struct ChildWithStreams<'a> {
    name: &'a str,
    exit_status: Option<Box<Future<Item = ExitStatus, Error = io::Error>>>,
    stdout_as_named_lines: Option<Box<Zip<Repeat<&'a str, io::Error>, Lines<BufReader<ChildStdout>>>>>,
}

impl<'a> ChildWithStreams<'a> {
    pub fn new(handle: &Handle, name: &'a str, params: &CommandParams) -> Self {
        let mut child = Command::new(params.bin())
            .args(params.args())
            .current_dir(params.directory())
            .stdout(Stdio::piped())
            .spawn_async(&handle)
            .expect("failed to spawn child process");

        let stdout = child.stdout().take().expect("couldn't capture stdout");
        let reader = BufReader::new(stdout);
        let lines = lines(reader);
        let named_lines = repeat(name).zip(lines);

        ChildWithStreams {
            name: name,
            exit_status: Some(Box::new(child)),
            stdout_as_named_lines: Some(Box::new(named_lines)),
        }
    }

    pub fn exit_status(&mut self) -> &mut Option<Box<Future<Item = ExitStatus, Error = io::Error>>> {
        &mut self.exit_status
    }

    pub fn stdout_as_named_lines(&mut self) -> &mut Option<Box<Zip<Repeat<&'a str, io::Error>, Lines<BufReader<ChildStdout>>>>> {
        &mut self.stdout_as_named_lines
    }
}
