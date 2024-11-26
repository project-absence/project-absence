use std::process;

use clap::Parser;

mod args;
mod config;
mod database;
mod debug;
mod events;
mod logger;
mod modules;
mod session;
mod state;

fn main() {
    config::create_file_if_not_existing();
    println!("Project Absence v{}", env!("CARGO_PKG_VERSION"));

    let args = args::Args::parse();
    let config = match args.parse_config() {
        Ok(config) => config,
        Err(e) => {
            logger::error("setup", e);
            process::exit(1);
        }
    };

    let (tx, rx) = flume::bounded::<events::Type>(100);
    let session = session::Session::new(args, config, tx, rx);
    session.register_config_modules();
    session.start();
}
