use crate::{config::Config, constants::STAPLE_CONFIG_FILE, error::StapleError};
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
            error!(
                "'{}' existed, please delete it and then continue",
                style(path).blue()
            );
            return Ok(());
        }
    }
    let config = Config::get_default_file();
    dbg!(&config);
    let string = toml::to_string(&config).expect("cannot serialize default config struct");
    std::fs::write(buf.join(STAPLE_CONFIG_FILE), string)?;
    std::fs::create_dir(buf.join("data"))?;
    std::fs::create_dir(buf.join("templates"))?;

    info!("init");
    Ok(())
}



#[cfg(test)]
mod test {
    use crate::command::init::init;
    use crate::constants::STAPLE_CONFIG_FILE;

    #[test]
    fn test_init() -> Result<(), Box< dyn std::error::Error>> {
        let dir = tempfile::tempdir()?.into_path();
        std::env::set_current_dir(&dir)?;
        init("./")?;

        assert!(dir.join(STAPLE_CONFIG_FILE).exists(), "cannot generate file Staple.toml");
        assert!(dir.join("data").is_dir(), "cannot find data folder");
        assert!(dir.join("templates").is_dir(), "cannot find templates folder");

        Ok(())
    }
}