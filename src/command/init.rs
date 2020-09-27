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
    let check_files = vec![STAPLE_CONFIG_FILE, "data", "templates"];
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
    let string = toml::to_string(&config).expect("cannot serialize default config struct");
    std::fs::write(buf.join(STAPLE_CONFIG_FILE), string)?;
    std::fs::create_dir(buf.join("data"))?;
    std::fs::create_dir(buf.join("templates"))?;
    std::fs::create_dir(buf.join("templates").join("staple"))?;

    info!("init");
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{command::init::init, constants::STAPLE_CONFIG_FILE};

    #[test]
    fn test_init() -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempfile::tempdir()?.into_path();
        std::env::set_current_dir(&dir)?;
        init("./")?;

        let check_point = vec![STAPLE_CONFIG_FILE, "data", "templates", "templates/staple"];

        for point in check_point {
            let buf = dir.join(point);
            assert!(buf.exists());
        }
        Ok(())
    }

    #[test]
    fn test_exist() -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempfile::tempdir()?.into_path();
        std::env::set_current_dir(&dir)?;

        let buf = dir.join(STAPLE_CONFIG_FILE);
        std::fs::write(buf, "exist test")?;
        init("./")?;

        assert!(!dir.join("data").exists());

        Ok(())
    }
}
