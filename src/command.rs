use crate::app::App;
use crate::error::StapleError;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::process::exit;
use std::time::{Duration, Instant};

use structopt::StructOpt;

use crate::server::ws::WsEvent;
use crate::server::Server;
use actix::prelude::*;
use actix::{
    Actor, ActorContext, Addr, AsyncContext, Context, Handler, StreamHandler, SystemRunner,
};
use actix_web::{web, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use file_lock::FileLock;
use std::path::Path;

const STAPLE_CONFIG_FILE: &'static str = "Staple.toml";

#[derive(StructOpt, Debug)]
#[structopt(name = "Staple")]
pub enum StapleCommand {
    Build,
    Develop,
}

impl StapleCommand {
    pub fn run(self) -> Result<(), StapleError> {
        match self {
            StapleCommand::Build => StapleCommand::build(),
            StapleCommand::Develop => StapleCommand::develop(),
        }
    }

    fn develop() -> Result<(), StapleError> {
        StapleCommand::config_exist()?;
        StapleCommand::build()?;

        let (addr, sys) = Server::start();

        let handle = std::thread::spawn(move || {
            let (tx, rx) = std::sync::mpsc::channel();
            let mut result: RecommendedWatcher =
                Watcher::new(tx, Duration::from_secs(2)).expect("cannot watch");
            result
                .watch("articles", RecursiveMode::Recursive)
                .expect("cannot watch articles");
            result
                .watch("templates", RecursiveMode::Recursive)
                .expect("cannot watch articles");
            result
                .watch("Staple.toml", RecursiveMode::Recursive)
                .expect("cannot watch articles");

            //                Ok(sys.run().expect("wrong on actix system run"))
            loop {
                match rx.recv() {
                    Ok(event) => {
                        println!("{:?}", event);
                        let result1 = App::load().expect("").render();
                        match result1 {
                            Ok(_) => {
                                println!("successfully");
                                addr.do_send(WsEvent::Refresh);
                            }
                            Err(e) => {
                                eprintln!("{}", dbg!(e));
                                exit(-1);
                            }
                        }
                    }
                    Err(e) => println!("watch error: {:?}", e),
                }
            }
        });
        sys.run().expect("");
        Ok(())
    }

    fn build() -> Result<(), StapleError> {
        StapleCommand::config_exist()?;
        let file_lock = match FileLock::lock("Staple.lock", true, true) {
            Ok(lock) => lock,
            Err(err) => panic!("Error getting write lock: {}", err),
        };

        App::load()?.render()
    }

    fn config_exist() -> Result<(), StapleError> {
        if Path::new(STAPLE_CONFIG_FILE).exists() {
            Ok(())
        } else {
            Err(StapleError::ConfigNotFound)
        }
    }
}
