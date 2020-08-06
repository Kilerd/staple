use std::{collections::HashMap, fs::File, io::Read, path::Path};
use std::ops::Deref;

use serde_derive::{Deserialize, Serialize};
use toml::Value;

use crate::error::StapleError;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub site: Site,
    pub statics: Vec<Statics>,
    pub extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigFile {
    pub site: Site,
    pub statics: Option<Vec<Statics>>,
    pub extra: HashMap<String, Value>,
}

impl Config {
    pub fn load_from_file() -> Result<Self, StapleError> {
        debug!("load config file");
        let config_content = std::fs::read_to_string("Staple.toml")?;
        let result: ConfigFile = toml::from_str(&config_content)?;
        Config::new_from_file(result)
    }

    pub fn new_from_file(config_file: ConfigFile) -> Result<Self, StapleError> {
        Ok(Self {
            site: config_file.site,
            statics: config_file.statics.unwrap_or_default(),
            extra: config_file.extra,
        })
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

impl Default for ConfigFile {
    fn default() -> Self {
        ConfigFile {
            site: Default::default(),
            extra: Default::default(),
            statics: None,
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
