extern crate futures;
extern crate tokio_core;
extern crate tokio_process;
extern crate tokio_io;
#[macro_use]
extern crate serde_derive;
extern crate toml;

extern crate loom;

use std::io::{self, Write};
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

use loom::CommandParams;
use loom::runner::run_commands;

type Config<'a> = HashMap<String, CommandConfig<'a>>;

#[derive(Deserialize)]
struct CommandConfig<'a> {
    name: &'a str,
    arg: &'a str
}

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

    let mut config_toml = String::new();
    File::open("./loomfile.toml").unwrap().read_to_string(&mut config_toml);

    let config: Config = toml::from_str(&config_toml).unwrap();

    let params_vec: Vec<CommandParams> = config
        .into_iter()
        .map(|(_, pair): (_, CommandConfig)| { CommandParams::new(pair.name, pair.arg) })
        .collect();

    run_commands(params_vec)
}
