use clap::{ArgMatches, Command};

use crate::errors::CliResult;
use crate::profile::Profile;

mod add;

pub struct CliContext {
    pub profile: Profile,
}

impl CliContext {
    pub fn new(profile: Profile) -> Self {
        CliContext { profile }
    }
}

pub type Exec = fn(&CliContext, &ArgMatches) -> CliResult;

pub fn commands() -> Vec<Command> {
    vec![add::cli()]
}

pub fn get_exec(cmd: &str) -> Option<Exec> {
    let f = match cmd {
        "add" => add::exec,
        _ => return None,
    };
    Some(f)
}
