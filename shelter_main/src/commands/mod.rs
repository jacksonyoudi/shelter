use anyhow;
use clap::{ArgMatches, Command};
use crate::settings::Settings;

mod hello;
mod serve;

mod migrate;

pub fn configure(command: Command) -> Command {
    command
        .subcommand(hello::configure())
        .subcommand(serve::configure())
}

pub fn handle(matches: &ArgMatches, _settings: &Settings) -> anyhow::Result<()> {
    hello::handle(matches, _settings)?;
    serve::handle(matches, _settings)?;
    Ok(())
}
