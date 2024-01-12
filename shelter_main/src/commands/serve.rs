use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use clap::{value_parser, Arg, ArgMatches, Command};
use crate::settings::Settings;
use tokio;
use axum_server;

use tower_http::trace::TraceLayer;
use crate::state::ApplicationState;

pub fn configure() -> Command {
    Command::new("serve").about("Start HTTP server").arg(
        Arg::new("port")
            .short('p')
            .long("port")
            .value_name("PORT")
            .help("TCP port to listen on")
            .default_value("8080")
            .value_parser(value_parser!(u16)),
    )
}

pub fn handle(matches: &ArgMatches, _settings: &Settings) -> anyhow::Result<()> {
    if let Some(matches) = matches.subcommand_matches("serve") {
        let port: u16 = *matches.get_one("port").unwrap_or(&8080);

        start_tokio(port, _settings)?;
    }

    Ok(())
}


fn start_tokio(port: u16, _settings: &Settings) -> anyhow::Result<()> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(
            async move {
                let state = Arc::new(ApplicationState::new(_settings)?);


                let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port);
                let routes = crate::api::configure(state)
                    .layer(TraceLayer::new_for_http());

                axum_server::Server::bind(addr)
                    .serve(routes.into_make_service())
                    .await?;


                Ok::<(), anyhow::Error>(())
            }
        )?;
    std::process::exit(0);
}