use crate::app::App;
use crate::article::Article;
use crate::error::StapleError;
use crate::template::Template;
use file_lock::FileLock;
use std::fs::File;
use std::process::exit;
use structopt::StructOpt;

mod app;
mod article;
mod config;
mod error;
mod template;

#[derive(StructOpt, Debug)]
#[structopt(name = "Staple")]
enum StapleCommand {
    Build,
}

impl StapleCommand {
    pub fn run(self) -> Result<(), StapleError> {
        match self {
            StapleCommand::Build => {
                let mut file_lock = match FileLock::lock("Staple.lock", true, true) {
                    Ok(lock) => lock,
                    Err(err) => panic!("Error getting write lock: {}", err),
                };

                App::load()?.render()
            }
        }
    }
}

fn main() {
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
}
