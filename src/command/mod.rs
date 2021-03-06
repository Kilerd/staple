use std::path::Path;

use structopt::StructOpt;

use crate::error::StapleError;

use crate::{
    command::add::AddOptions,
    constants::{STAPLE_CONFIG_FILE, STAPLE_LOCK_FILE},
    util::lock::LockFile,
};

pub mod add;
pub mod build;
pub mod develop;
pub mod init;
pub mod list;
pub mod new;

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
    Develop {
        /// port of developing server listens on
        #[structopt(short, long, default_value = "8000", env = "STAPLE_DEVELOP_PORT")]
        port: u16,
    },
    /// add new article
    Add(AddOptions),

    /// show all information of staple project
    List,
}

impl StapleCommand {
    pub fn run(self) -> Result<(), StapleError> {
        let path = ".";
        match self {
            StapleCommand::New { path, title, force } => new::new(path, title, force),
            StapleCommand::Init => init::init(&path),
            StapleCommand::Build => build::build(path, false),
            StapleCommand::Develop { port } => develop::develop(&path, port),
            StapleCommand::List => {
                StapleCommand::check_config_file_exist(&path)?;
                list::command(&path)
            }

            StapleCommand::Add(options) => add::add(&path, options),
        }
    }

    #[inline]
    fn config_file_exist(path: impl AsRef<Path>) -> bool {
        path.as_ref().join(STAPLE_CONFIG_FILE).exists()
    }

    fn check_config_file_exist(path: impl AsRef<Path>) -> Result<(), StapleError> {
        if StapleCommand::config_file_exist(path) {
            Ok(())
        } else {
            Err(StapleError::ConfigNotFound)
        }
    }

    fn lock_file(path: impl AsRef<Path>) -> Result<LockFile, StapleError> {
        let buf = path.as_ref().join(STAPLE_LOCK_FILE);
        let lock_file = LockFile::new(buf)?;
        info!("Preparing to lock file...");
        lock_file.lock_file()?;
        Ok(lock_file)
    }
}
