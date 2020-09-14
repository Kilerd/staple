use std::{collections::HashMap, path::Path};

use serde_derive::{Deserialize, Serialize};
use toml::Value;

use crate::error::StapleError;
use serde::export::Formatter;
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub site: Site,
    #[serde(default)]
    pub statics: Vec<Statics>,
    pub extra: HashMap<String, Value>,
    #[serde(default)]
    pub hook: Hook,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
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

impl Config {
    pub fn load_from_file() -> Result<Self, StapleError> {
        debug!("load config file");
        let config_content = std::fs::read_to_string("Staple.toml")?;
        let result: Config = toml::from_str(&config_content)?;
        Ok(dbg!(result))
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
    pub fn get_default_file() -> Config {
        Config::default()
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            site: Default::default(),
            statics: vec![],
            extra: Default::default(),
            hook: Hook {
                before_build: vec![],
                after_build: vec![],
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
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
            title: "".to_string(),
            subtitle: "".to_string(),
            description: "".to_string(),
            keywords: vec![],
            author: "".to_string(),
            email: "".to_string(),
            utc_offset: 800,
            theme: "rubble".to_string(),
            domain: "".to_string(),
            domain_root: "".to_string(),
            default_template: "article.html".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Url {
    pub url: String,
    pub root: String,
}

impl Default for Url {
    fn default() -> Self {
        Self {
            url: "http://localhost:8000".to_string(),
            root: "/".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Statics {
    pub from: String,
    pub to: String,
}

#[cfg(test)]
mod test {
    use crate::config::HookLine;

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
}
