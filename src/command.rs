use crate::config::Config;
use crate::{
    app::App,
    error::StapleError,
    server::{ws::WsEvent, Server},
};
use console::style;
use file_lock::FileLock;
use notify::{DebouncedEvent as Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::default::Default;
use std::{
    path::Path,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    time::{Duration, Instant},
};
use structopt::StructOpt;

const STAPLE_CONFIG_FILE: &'static str = "Staple.toml";

#[derive(StructOpt, Debug)]
#[structopt(name = "Staple")]
pub enum StapleCommand {
    Init,
    Build,
    Develop,
}

impl StapleCommand {
    pub fn run(self) -> Result<(), StapleError> {
        match self {
            StapleCommand::Init => StapleCommand::init(),
            StapleCommand::Build => StapleCommand::build(),
            StapleCommand::Develop => StapleCommand::develop(),
        }
    }

    /// init target folder as staple project structure
    /// check whether `Staple.toml` exist or not
    /// generate `Staple.toml` config file
    /// create folders `articles`, `templates`
    /// put default template files
    fn init() -> Result<(), StapleError> {
        let check_files = vec![STAPLE_CONFIG_FILE, "articles", "templates"];
        for path in check_files {
            if Path::new(path).exists() {
                println!(
                    "{} '{}' existed, please delete it and then continue",
                    style("ERROR").red(),
                    style(path).blue()
                );
                return Ok(());
            }
        }
        let config = Config::default();
        let string = toml::to_string(&config).expect("cannot serialize default config struct");
        std::fs::write(STAPLE_CONFIG_FILE, string)?;
        std::fs::create_dir("articles")?;
        std::fs::create_dir("templates")?;

        println!("init");
        Ok(())
    }

    fn develop() -> Result<(), StapleError> {
        StapleCommand::check_config_file_exist()?;
        StapleCommand::build()?;

        let has_new_file_event = Arc::new(AtomicBool::new(false));
        let _is_building = Arc::new(Mutex::new(false));

        let (addr, sys) = Server::start();

        let file_event_flag_for_watcher = has_new_file_event.clone();
        let _watcher_thread = std::thread::spawn(move || {
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
            let _instant = Arc::new(AtomicBool::new(false));
            let _instant1 = Instant::now();

            // 有文件事件来的时候就把 `should_update_flag` 设置为 true
            // 循环监听，如果是true 就 build，完成后休眠100ms， build 之前先设置标识为为 false
            loop {
                match rx.recv() {
                    Ok(event) => match &event {
                        Event::Chmod(_)
                        | Event::Create(_)
                        | Event::Write(_)
                        | Event::Rename(_, _) => {
                            info!("get an file event, {:?}", event);
                            file_event_flag_for_watcher.store(true, Ordering::Relaxed);
                        }
                        _ => {}
                    },
                    Err(e) => println!("watch error: {:?}", e),
                }
            }
        });

        let file_event_flag_for_builder = has_new_file_event.clone();
        let _handle = std::thread::spawn(move || loop {
            let need_build =
                file_event_flag_for_builder.compare_and_swap(true, false, Ordering::Relaxed);
            if need_build {
                info!("build app");
                StapleCommand::build();
                addr.do_send(WsEvent::Refresh);
            }
            std::thread::sleep(Duration::from_secs(1));
        });

        sys.run().expect("");
        Ok(())
    }

    fn build() -> Result<(), StapleError> {
        StapleCommand::check_config_file_exist()?;
        let _file_lock = match FileLock::lock("Staple.lock", true, true) {
            Ok(lock) => lock,
            Err(err) => panic!("Error getting write lock: {}", err),
        };

        App::load()?.render()
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
