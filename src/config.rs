use std::collections::HashMap;

#[derive(Deserialize,Copy,Clone)]
pub struct CommandParams<'a> {
    command: &'a str,
     #[serde(default = "CommandParams::default_directory")]
    directory: &'a str
}

impl<'a> CommandParams<'a> {
    pub fn default_directory() -> &'a str {
        "."
    }

    pub fn bin(self) -> &'a str {
        self.command.split(' ').nth(0).unwrap()
    }

    pub fn args(self) -> Vec<&'a str> {
        self.command.split(' ').skip(1).collect()
    }

    pub fn directory(self) -> &'a str {
        self.directory
    }
}

pub type Config<'a> = HashMap<String, CommandParams<'a>>;
