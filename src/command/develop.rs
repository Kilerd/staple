use crate::command::StapleCommand;
use crate::error::StapleError;
use crate::server::ws::WsEvent;
use crate::server::Server;
use notify::DebouncedEvent as Event;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

pub(crate) fn develop() -> Result<(), StapleError> {
    StapleCommand::check_config_file_exist()?;
    crate::command::build::build()?;

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
                    Event::Chmod(_) | Event::Create(_) | Event::Write(_) | Event::Rename(_, _) => {
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
            crate::command::build::build();
            addr.do_send(WsEvent::Refresh);
        }
        std::thread::sleep(Duration::from_secs(1));
    });

    sys.run().expect("");
    Ok(())
}
