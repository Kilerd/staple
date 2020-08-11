use std::time::Duration;

#[cfg(windows)]
pub const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
pub const LINE_ENDING: &str = "\n";

pub const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
pub const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub const STAPLE_CONFIG_FILE: &str = "Staple.toml";
pub const STAPLE_LOCK_FILE: &str = "Staple.lock";

pub const DESCRIPTION_SEPARATOR: &str = "<!--more-->";

pub const LIVE_RELOAD_CODE: &str = include_str!("../data/live_reload.html");
