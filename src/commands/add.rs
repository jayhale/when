use crate::commands::CliContext;
use crate::errors::CliResult;
use clap::{Arg, ArgAction, ArgMatches, Command};

pub fn cli() -> Command {
    Command::new("add")
        .about("Add a calendar connection")
        .arg(
            Arg::new("name")
                .value_name("NAME")
                .action(ArgAction::Set)
                .required(true)
                .help("Name of the calendar"),
        )
        .arg(
            Arg::new("ics_url")
                .short('i')
                .long("ics")
                .value_name("URL")
                .action(ArgAction::Set)
                .required(true)
                .help("URL to an ICS file with the calendar you want to add"),
        )
}

pub fn exec(ctx: &CliContext, matches: &ArgMatches) -> CliResult {
    let name: &String = matches.get_one("name").expect("`name` is required");
    let ics_url: &String = matches.get_one("ics_url").expect("`ics_url` is required");

    let mut profile = ctx.profile.clone();
    profile.add_ics_calendar(name.clone(), ics_url.clone());
    profile.save();
    return Ok(());
}
