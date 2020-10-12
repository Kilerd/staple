use crate::{config::Config, constants::STAPLE_CONFIG_FILE, error::StapleError};
use colored::Colorize;
use console::style;
use std::path::Path;

/// init target folder as staple project structure
/// check whether `Staple.toml` exist or not
/// generate `Staple.toml` config file
/// create folders `articles`, `templates`
/// put default template files
pub(crate) fn init(path: impl AsRef<Path>) -> Result<(), StapleError> {
    let buf = path.as_ref();
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
    info!("Creating file {}", STAPLE_CONFIG_FILE.blue());
    let config = Config::get_default_file();
    let string = toml::to_string(&config).expect("cannot serialize default config struct");
    std::fs::write(buf.join(STAPLE_CONFIG_FILE), string)?;
    info!("Creating folder {}", "data".blue());
    std::fs::create_dir(buf.join("data"))?;
    info!("Creating folder {}", "template".blue());
    std::fs::create_dir(buf.join("templates"))?;
    info!("Creating template {}", "staple".blue());
    std::fs::create_dir(buf.join("templates").join("staple"))?;
    info!("Creating template {}", "staple/article".blue());
    std::fs::write(
        buf.join("templates").join("staple").join("article.html"),
        "{{ page.content.html | safe }}",
    )?;
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{command::init::init, constants::STAPLE_CONFIG_FILE, test::setup};

    #[test]
    fn test_init() -> Result<(), Box<dyn std::error::Error>> {
        let dir = setup();
        init(&dir)?;

        let check_point = vec![
            STAPLE_CONFIG_FILE,
            "data",
            "templates",
            "templates/staple",
            "templates/staple/article.html",
        ];

        for point in check_point {
            let buf = dir.join(point);
            assert!(buf.exists());
        }
        Ok(())
    }

    #[test]
    fn test_exist() -> Result<(), Box<dyn std::error::Error>> {
        let dir = setup();

        let buf = dir.join(STAPLE_CONFIG_FILE);
        std::fs::write(buf, "exist test")?;
        init(&dir)?;

        assert!(!dir.join("data").exists());

        Ok(())
    }
}
