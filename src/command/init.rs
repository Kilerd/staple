use crate::config::Config;
use crate::constants::STAPLE_CONFIG_FILE;
use crate::error::StapleError;
use console::style;
use std::path::Path;

/// init target folder as staple project structure
/// check whether `Staple.toml` exist or not
/// generate `Staple.toml` config file
/// create folders `articles`, `templates`
/// put default template files
pub(crate) fn init(path: &str) -> Result<(), StapleError> {
    let buf = Path::new(".").join(path);
    let check_files = vec![STAPLE_CONFIG_FILE, "articles", "templates"];
    for path in check_files {
        if buf.join(path).exists() {
            info!(
                "{} '{}' existed, please delete it and then continue",
                style("ERROR").red(),
                style(path).blue()
            );
            return Ok(());
        }
    }
    let config = Config::get_default_file();
    let string = toml::to_string(&config).expect("cannot serialize default config struct");
    std::fs::write(buf.join(STAPLE_CONFIG_FILE), string)?;
    std::fs::create_dir(buf.join("articles"))?;
    std::fs::create_dir(buf.join("pages"))?;
    std::fs::create_dir(buf.join("templates"))?;

    info!("init");
    Ok(())
}
