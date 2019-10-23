#[macro_use]
extern crate pest_derive;

#[macro_use]
extern crate log;

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::hash::Hash;
use std::io::Read;
use std::process::exit;

use file_lock::FileLock;

use structopt::StructOpt;

use crate::app::App;
use crate::article::Article;
use crate::error::StapleError;
use crate::template::Template;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::time::Duration;

mod app;
mod article;
mod config;
mod error;
mod template;

#[derive(StructOpt, Debug)]
#[structopt(name = "Staple")]
enum StapleCommand {
    Build,
    Develop,
}

impl StapleCommand {
    pub fn run(self) -> Result<(), StapleError> {
        match self {
            StapleCommand::Build => {
                let file_lock = match FileLock::lock("Staple.lock", true, true) {
                    Ok(lock) => lock,
                    Err(err) => panic!("Error getting write lock: {}", err),
                };

                App::load()?.render()
            }
            StapleCommand::Develop => {
                let (tx, rx) = std::sync::mpsc::channel();
                let mut result: RecommendedWatcher =
                    Watcher::new(tx, Duration::from_secs(1)).expect("cannot watch");
                result
                    .watch("articles", RecursiveMode::Recursive)
                    .expect("cannot watch articles");
                result
                    .watch("templates", RecursiveMode::Recursive)
                    .expect("cannot watch articles");
                result
                    .watch("Staple.toml", RecursiveMode::Recursive)
                    .expect("cannot watch articles");

                loop {
                    match rx.recv() {
                        Ok(event) => {
                            println!("{:?}", event);
                            App::load()?.render();
                        }
                        Err(e) => println!("watch error: {:?}", e),
                    }
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init_timed();

    let opt: StapleCommand = StapleCommand::from_args();
    let result = opt.run();
    match result {
        Ok(_) => {
            println!("successfully");
        }
        Err(e) => {
            eprintln!("{}", dbg!(e));
            exit(-1);
        }
    }

    Ok(())
}
