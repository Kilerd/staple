use crate::{
    command::StapleCommand,
    config::Config,
    error::StapleError,
    server::{ws::WsEvent, Server},
};
use notify::{DebouncedEvent as Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::{
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

pub(crate) fn develop(path: impl AsRef<Path>) -> Result<(), StapleError> {
    StapleCommand::check_config_file_exist(&path)?;
    crate::command::build::build(&path, true)?;

    let has_new_file_event = Arc::new(AtomicBool::new(false));
    let _is_building = Arc::new(AtomicBool::new(false));

    let (addr, sys) = Server::start();

    let file_event_flag_for_watcher = has_new_file_event.clone();
    let _watcher_thread = std::thread::spawn(move || {
        let (tx, rx) = std::sync::mpsc::channel();
        let mut result: RecommendedWatcher =
            Watcher::new(tx, Duration::from_secs(2)).expect("cannot watch");
        info!("Watching ./data");
        result
            .watch("data", RecursiveMode::Recursive)
            .expect("cannot watch articles");
        info!("Watching ./templates");
        result
            .watch("templates", RecursiveMode::Recursive)
            .expect("cannot watch articles");
        info!("Watching ./Staple.toml");
        result
            .watch("Staple.toml", RecursiveMode::Recursive)
            .expect("cannot watch articles");
        let exclusive_list: Vec<PathBuf> = Config::load_from_file("./")
            .expect("cannot load config")
            .watch
            .exclusive
            .into_iter()
            .map(|p| {
                info!("Unwatching {}", &p);
                Path::new(&p).canonicalize().expect("invalid unwatch path")
            })
            .collect();

        // 有文件事件来的时候就把 `should_update_flag` 设置为 true
        // 循环监听，如果是true 就 build，完成后休眠100ms， build 之前先设置标识为为 false
        loop {
            match rx.recv() {
                Ok(event) => match &event {
                    Event::Chmod(source)
                    | Event::Create(source)
                    | Event::Write(source)
                    | Event::Rename(source, _) => {
                        let event_path = source
                            .canonicalize()
                            .expect("cannot canonicalize event path");
                        let is_exclusive = exclusive_list
                            .iter()
                            .any(|exclusive| event_path.strip_prefix(exclusive).is_ok());
                        if is_exclusive {
                            debug!("Get exclusive file event: {:?}", event);
                        } else {
                            info!("get an file event, {:?}", event);
                            file_event_flag_for_watcher.store(true, Ordering::Relaxed);
                        }
                    }
                    _ => {}
                },
                Err(e) => info!("watch error: {:?}", e),
            }
        }
    });

    let file_event_flag_for_builder = has_new_file_event;
    let buf = path.as_ref().to_path_buf();
    let _handle = std::thread::spawn(move || loop {
        let need_build =
            file_event_flag_for_builder.compare_and_swap(true, false, Ordering::Relaxed);
        if need_build {
            info!("build app");
            info!("build stage is triggered by file event.");
            let result1 = crate::command::build::build(buf.clone(), true);
            match result1 {
                Ok(_) => info!("build successfully"),
                Err(e) => error!("fail to build due to {}", e),
            }
            addr.do_send(WsEvent::Refresh);
        }
        std::thread::sleep(Duration::from_secs(1));
    });
    info!("developing server is listening on http://127.0.0.1:8000");
    sys.run().expect("");
    Ok(())
}
