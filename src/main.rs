#![warn(clippy::dbg_macro)]
#[macro_use]
extern crate pest_derive;

#[macro_use]
extern crate log;

use crate::command::StapleCommand;
use env_logger::Env;
use std::{io::Write, process::exit};
use structopt::StructOpt;

mod app;
mod command;
mod config;
mod constants;
mod error;
mod server;
mod template;
mod util;

mod data;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::from_env(Env::default().default_filter_or("info"))
        .format(|buf, record| {
            let level = buf.default_styled_level(record.level());
            writeln!(buf, "{:>5} {}", level, record.args())
        })
        .init();

    let opt: StapleCommand = StapleCommand::from_args();
    let result = opt.run();
    if let Err(e) = result {
        error!("Error: {}", e);
        exit(1);
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    pub fn setup() -> PathBuf {
        let _ = env_logger::builder().is_test(true).try_init().is_ok();
        tempfile::tempdir()
            .expect("Cannot init temp dir")
            .into_path()
    }
}
