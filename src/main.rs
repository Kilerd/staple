
use file_lock::FileLock;
use structopt::StructOpt;
use std::fs::File;
use crate::article::Article;
use crate::template::Template;

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
                template.render(articles);
                println!("build successfully");
            }
        }
    }
}

fn main() {
    let opt:StapleCommand = StapleCommand::from_args();
    opt.run();
}