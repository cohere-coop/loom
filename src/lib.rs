extern crate futures;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_process;
extern crate tokio_signal;
#[macro_use]
extern crate serde_derive;
extern crate toml;

mod runner;
mod config;

use std::io::prelude::*;
use std::fs::File;

pub fn run_from_config(filename: &str) -> Result<(), (std::io::Error)> {
    let mut config_toml = String::new();
    File::open(filename).unwrap().read_to_string(&mut config_toml).expect("could not open Loomfile");

    let config: config::Config = toml::from_str(&config_toml).unwrap();

    runner::run_commands(config)
}
