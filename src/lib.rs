extern crate futures;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_process;
extern crate tokio_signal;
#[macro_use]
extern crate serde_derive;
extern crate toml;

pub mod runner;

use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use std::process::Command;

#[derive(Deserialize,Copy,Clone)]
pub struct CommandParams<'a> {
    command: &'a str
}

impl<'a> CommandParams<'a> {
    pub fn bin(self) -> &'a str {
        self.command.split(' ').nth(0).unwrap()
    }

    pub fn args(self) -> Vec<&'a str> {
        self.command.split(' ').skip(1).collect()
    }
}

pub type Config<'a> = HashMap<String, CommandParams<'a>>;

pub fn run_from_config(filename: &str) -> Result<(), (std::io::Error)> {
    let mut config_toml = String::new();
    File::open(filename).unwrap().read_to_string(&mut config_toml).expect("could not open Loomfile");

    let mut config: Config = toml::from_str(&config_toml).unwrap();
    // let params_vec: Vec<&mut CommandParams> = config.values_mut().collect();

    runner::run_commands(config)
}
