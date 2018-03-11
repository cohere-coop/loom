extern crate futures;
extern crate tokio_core;
extern crate tokio_process;
extern crate tokio_io;

extern crate loom;

use std::io::{self, Write};

use loom::run_from_config;

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
    run_from_config("./loomfile.toml")
}
