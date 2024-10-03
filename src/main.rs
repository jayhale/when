use clap::{command, value_parser, Arg, ArgAction};
use std::path::PathBuf;

use crate::commands::CliContext;

mod commands;
mod errors;
mod ical;
mod profile;

fn main() {
    let matches = command!()
        .subcommands(commands::commands())
        .arg(
            Arg::new("profile")
                .short('p')
                .long("profile")
                .value_name("FILE")
                .action(ArgAction::Set)
                .default_value("profile.toml")
                .value_parser(value_parser!(PathBuf))
                .help("Location of your profile file"),
        )
        .get_matches();

    let profile_file = matches.get_one::<PathBuf>("profile").unwrap().clone();
    let profile = profile::Profile::from_file(&profile_file);

    let ctx = CliContext::new(profile);

    match matches.subcommand() {
        Some((cmd, matches)) => {
            // TODO: Handle errors
            let exec = commands::get_exec(cmd).unwrap();
            exec(&ctx, matches).unwrap();
        }
        _ => {
            todo!("Default to returning results")
        }
    }
}
