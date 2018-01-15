#[macro_use]
extern crate log;
extern crate simplelog;
extern crate getopts;

mod tags;
mod search;
mod server;


use simplelog::*;
use std::fs::File;

use server::Server;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    init_logging();
    run_server();
}

fn init_logging() {
    CombinedLogger::init(
        vec![
            TermLogger::new(LogLevelFilter::Trace, Config::default()).unwrap(),
            WriteLogger::new(LogLevelFilter::Trace, Config::default(), File::create("enigma.log").unwrap()),
        ]
    ).unwrap();    
}

fn run_server() {
    info!("Running enigma {}", VERSION);    
    Server::start_with_config_file("enigma.yaml");
}
