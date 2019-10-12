
use file_lock::FileLock;
use structopt::StructOpt;
use std::fs::File;
use crate::article::Article;
use crate::template::Template;
use std::process::exit;

mod article;
mod template;
mod error;

#[derive(StructOpt, Debug)]
#[structopt(name="Staple")]
enum StapleCommand {
    Build
}

impl StapleCommand {
    pub fn run(self) {
        match self {
            StapleCommand::Build => {
                let mut file_lock = match FileLock::lock("Staple.lock", true, true) {
                    Ok(lock) => lock,
                    Err(err) => panic!("Error getting write lock: {}", err),
                };
                let articles = Article::load_all_article();
                let template = Template::new("rubble".to_string());
                let result = template.render(articles);
                match result {
                    Ok(_) => {
                        println!("build successfully");
                    },
                    Err(e) => {
                        dbg!(e);
                        exit(-1);
                    }
                }

            }
        }
    }
}

fn main() {
    let opt:StapleCommand = StapleCommand::from_args();
    opt.run();
}