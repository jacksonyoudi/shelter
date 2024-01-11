use clap::{Arg, Command};
use anyhow;
use dotenv::dotenv;

use shelter_main::{commands, settings};

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
    let matches = command.get_matches();
    let config_location = matches
        .get_one::<String>("config")
        .map(|s| s.as_str())
        .unwrap_or("");


    let st = settings::Settings::new(config_location, "SHELTER")?;


    // 下面是消耗的
    // println!(
    //     "db url: {}",
    //     st
    //         .database
    //         .url
    //         .unwrap_or("missing database url".to_string())
    // );
    //
    //
    // println!(
    //     "log level: {}",
    //     st.logging.log_level.unwrap_or("info".to_string())
    // );

    commands::handle(&matches, &st)?;


    Ok(())
}