#[macro_use]
extern crate pest_derive;

#[macro_use]
extern crate log;

use actix::prelude::*;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::hash::Hash;
use std::io::Read;
use std::process::exit;

use file_lock::FileLock;

use structopt::StructOpt;

use crate::app::App;
use crate::article::Article;
use crate::command::StapleCommand;
use crate::error::StapleError;
use crate::template::Template;
use actix::{Actor, StreamHandler};
use actix_web::{web, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::Mutex;
use std::time::{Duration, Instant};

mod app;
mod article;
mod command;
mod config;
mod error;
mod server;
mod template;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
