#[macro_use]
extern crate pest_derive;

use crate::app::App;
use crate::article::Article;
use crate::error::StapleError;
use crate::template::Template;
use file_lock::FileLock;
use lalrpop_util::lalrpop_mod;
use pest::Parser;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::process::exit;
use structopt::StructOpt;

mod app;
mod article;
mod ast;
mod config;
mod error;
mod template;

lalrpop_mod!(
    #[allow(clippy::all)]
    article_parser
);

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

#[derive(Parser)]
#[grammar = "article.pest"] // relative to src
struct MyParser;

fn main() -> Result<(), Box<dyn Error>> {
    //    let opt: StapleCommand = StapleCommand::from_args();
    //    let result = opt.run();
    //    match result {
    //        Ok(_) => {
    //            println!("successfully");
    //        }
    //        Err(e) => {
    //            eprintln!("{}", dbg!(e));
    //            exit(-1);
    //        }
    //    }

    let mut file = std::fs::File::open("test.md")?;
    let mut string = String::new();
    file.read_to_string(&mut string)?;

    let content;

    let parse1 = MyParser::parse(Rule::article, &string);
    let x = parse1.expect("").next().unwrap();
    for pair in x.into_inner() {
        match pair.as_rule() {
            Rule::meta => {}
            Rule::content => content = String::from(pair.into_inner().next().as_str()),
        }
    }

    dbg!(conent);

    Ok(())
}
