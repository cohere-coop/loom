extern crate futures;
extern crate tokio_core;
extern crate tokio_process;
extern crate tokio_io;

extern crate loom;

use std::io::{self, Write};

use loom::CommandParams;
use loom::runner::run_commands;

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
    run_commands(vec![CommandParams::new("ping", "wecohere.com"), CommandParams::new("ping", "google.com")])
}
