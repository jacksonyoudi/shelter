use clap::{Arg, Command};
use anyhow;
use dotenv::dotenv;

use shelter_main::commands;

pub fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let command = Command::new("Dog Shelter  sample application")
        .version("1.0")
        .author("liangchangyoujackson@gmail.com")
        .about("A sample application to experiment with Rust-based microservices")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .help("Configuration file location")
                .default_value("config.json")
        );

    let command = commands::configure(command);
    let _matches = command.get_matches();
    commands::handle(&_matches)?;
    Ok(())
}