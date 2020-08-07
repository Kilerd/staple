use std::time::Duration;

#[cfg(windows)]
pub const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
pub const LINE_ENDING: &'static str = "\n";

pub const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
pub const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub const STAPLE_CONFIG_FILE: &'static str = "Staple.toml";

pub const DESCRIPTION_SEPARATOR: &'static str = "<!--more-->";

pub const LIVE_RELOAD_CODE: &'static str = include_str!("../data/live_reload.html");
