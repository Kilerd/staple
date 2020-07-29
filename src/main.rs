#[macro_use]
extern crate pest_derive;

#[macro_use]
extern crate log;

use crate::command::StapleCommand;
use std::process::exit;
use structopt::StructOpt;

mod app;
mod command;
mod config;
mod constants;
mod error;
mod server;
mod template;

mod data;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init_timed();

    let opt: StapleCommand = StapleCommand::from_args();
    let result = opt.run();
    if let Err(e) = result {
        eprintln!("Error: {}", e);
        exit(1);
    }
    Ok(())
}
