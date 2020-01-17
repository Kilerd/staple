use crate::article::Article;
use crate::config::Config;
use crate::template::Template;
use crate::{
    app::App,
    error::StapleError,
    server::{ws::WsEvent, Server},
};
use console::style;
use file_lock::FileLock;
use notify::{DebouncedEvent as Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::default::Default;

use crate::command::article::ArticleCommand;
use crate::constants::STAPLE_CONFIG_FILE;
use std::{
    path::Path,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    time::{Duration, Instant},
};
use structopt::StructOpt;

pub mod article;
pub mod build;
pub mod develop;
pub mod init;
pub mod new;
pub mod page;

#[derive(StructOpt, Debug)]
#[structopt(name = "Staple")]
pub enum StapleCommand {
    /// create a new folder and then init it as Staple project.
    New {
        /// folder name
        path: String,
        /// specific Staple project's title, default is Staple
        #[structopt(long)]
        title: Option<String>,
        /// force to delete exist folder if existed, then create a new one and initialize.
        #[structopt(short, long)]
        force: bool,
    },
    /// init current folder as Staple project.
    Init,
    /// build
    Build,
    /// start the develop server listening on local with live-reload
    Develop,
    /// modification of articles
    Article(ArticleCommand),
}

impl StapleCommand {
    pub fn run(self) -> Result<(), StapleError> {
        match self {
            StapleCommand::New { path, title, force } => new::new(path, title, force),
            StapleCommand::Init => init::init("."),
            StapleCommand::Build => build::build(),
            StapleCommand::Develop => develop::develop(),
            StapleCommand::Article(article_command) => {
                StapleCommand::check_config_file_exist()?;
                article_command.run()
            }
        }
    }

    fn config_file_exist() -> bool {
        Path::new(STAPLE_CONFIG_FILE).exists()
    }

    fn check_config_file_exist() -> Result<(), StapleError> {
        if Path::new(STAPLE_CONFIG_FILE).exists() {
            Ok(())
        } else {
            Err(StapleError::ConfigNotFound)
        }
    }
}
