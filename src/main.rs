
use file_lock::FileLock;
use structopt::StructOpt;
use std::fs::File;
use crate::article::Article;
use crate::template::Template;
use std::process::exit;
use crate::app::App;
use crate::error::StapleError;

mod article;
mod template;
mod error;
mod app;
mod config;

#[derive(StructOpt, Debug)]
#[structopt(name="Staple")]
enum StapleCommand {
    Build
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
    let opt:StapleCommand = StapleCommand::from_args();
    let result = opt.run();
    match result {
        Ok(_) => {
            println!("successfully");
        }
        Err(e) => {
            match e {
                StapleError::CanNotOperateDotRenderFolder => {
                    panic!(" 无法操作零时文件夹 .render");
                }
                StapleError::IoError(e) => {
                    panic!("{}", e.to_string());
                }
                StapleError::ConfigError(e) => {
                    panic!("{}", e.to_string());
                }
                StapleError::RenderError(e) => {
                    panic!("{}", e.to_string());
                }
            }
        }
    }
}