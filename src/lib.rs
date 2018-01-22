extern crate futures;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_process;

pub mod runner;

pub struct CommandParams<'a> {
    pub bin: &'a str,
    pub arg: &'a str,
}

impl<'a> CommandParams<'a> {
    pub fn new(bin: &'a str, arg: &'a str) -> Self {
        CommandParams { bin: bin, arg: arg }
    }
}
