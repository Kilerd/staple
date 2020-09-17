use std::{collections::HashMap, path::Path};

use serde_derive::{Deserialize, Serialize};
use toml::Value;

use crate::{constants::STAPLE_CONFIG_FILE, error::StapleError};
use serde::export::Formatter;
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Config {
    pub site: Site,
    #[serde(default)]
    pub statics: Vec<Statics>,
    #[serde(default)]
    pub hook: Hook,
    pub extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Hook {
    #[serde(default)]
    pub before_build: Vec<HookLine>,
    #[serde(default)]
    pub after_build: Vec<HookLine>,
}

impl Default for Hook {
    fn default() -> Self {
        Hook {
            before_build: vec![],
            after_build: vec![],
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum HookLine {
    Command(String),
    TargetDir { dir: String, command: String },
}

impl HookLine {
    pub fn to_dir(&self) -> String {
        match self {
            HookLine::Command(_) => String::from("./"),
            HookLine::TargetDir { dir, command: _ } => dir.to_string(),
        }
    }
    pub fn to_cmd(&self) -> String {
        match self {
            HookLine::Command(cmd) => cmd.to_string(),
            HookLine::TargetDir { dir: _, command } => command.to_string(),
        }
    }
}

impl Display for HookLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.to_dir(), self.to_cmd())
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ConfigFile {
    pub site: Site,
    pub statics: Option<Vec<Statics>>,
    pub extra: HashMap<String, Value>,
    pub hook: Hook,
}

impl Default for ConfigFile {
    fn default() -> Self {
        ConfigFile {
            site: Default::default(),
            statics: None,
            extra: Default::default(),
            hook: Default::default(),
        }
    }
}

impl Config {
    pub fn load_from_file(path: impl AsRef<Path>) -> Result<Self, StapleError> {
        debug!("load config file");
        let config_file_path = path.as_ref().join(STAPLE_CONFIG_FILE);
        let config_content = std::fs::read_to_string(config_file_path)?;
        let result: Config = toml::from_str(&config_content)?;
        Ok(result)
    }

    pub fn get_theme(&self) -> Result<String, StapleError> {
        let empty_theme = self.site.theme.eq("");
        let theme_exist = !Path::new("templates")
            .join(self.site.theme.clone())
            .exists();
        if empty_theme || theme_exist {
            Err(StapleError::ThemeNotFound(self.site.theme.clone()))
        } else {
            Ok(self.site.theme.clone())
        }
    }
    pub fn get_default_file() -> ConfigFile {
        ConfigFile::default()
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            site: Default::default(),
            statics: vec![],
            hook: Hook {
                before_build: vec![],
                after_build: vec![],
            },
            extra: Default::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Site {
    pub title: String,
    pub subtitle: String,
    pub description: String,
    pub keywords: Vec<String>,
    pub author: String,
    pub email: String,
    pub utc_offset: i16,
    pub theme: String,
    pub domain: String,
    pub domain_root: String,
    pub default_template: String,
}

impl Default for Site {
    fn default() -> Self {
        Self {
            title: "Staple Site".to_string(),
            subtitle: "".to_string(),
            description: "".to_string(),
            keywords: vec![],
            author: "".to_string(),
            email: "".to_string(),
            utc_offset: 800,
            theme: "staple".to_string(),
            domain: "".to_string(),
            domain_root: "".to_string(),
            default_template: "article.html".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Statics {
    pub from: String,
    pub to: String,
}

#[cfg(test)]
mod test {
    use crate::config::{Config, ConfigFile, HookLine};

    #[test]
    fn test_hook_display() {
        assert_eq!("[./] ls", HookLine::Command("ls".to_string()).to_string());
        assert_eq!(
            "[data] ls",
            HookLine::TargetDir {
                dir: "data".to_string(),
                command: "ls".to_string()
            }
            .to_string()
        );
    }

    #[test]
    fn test_config_file_site_default() {
        let config = Config::default();
        let site = config.site;
        assert_eq!("Staple Site", site.title);
        assert_eq!("", site.author);
        assert_eq!("staple", site.theme);
        assert_eq!("", site.domain_root);
        assert_eq!("article.html", site.default_template);
        assert_eq!(800, site.utc_offset);
    }

    #[test]
    fn test_config_file_hook_default() {
        let config = Config::default();
        assert!(config.hook.before_build.is_empty());
        assert!(config.hook.after_build.is_empty());
    }

    #[test]
    fn test_config_file_statics_default() {
        let config = Config::default();
        assert!(config.statics.is_empty());
    }

    #[test]
    fn test_config_file_extra_default() {
        let config = Config::default();
        assert!(config.extra.is_empty());
    }

    #[test]
    fn test_config_default_generator() {
        let config = Config::get_default_file();
        assert_eq!(ConfigFile::default(), config);
    }
}
