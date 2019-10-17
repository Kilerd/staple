use std::collections::HashMap;
use std::fs::File;
use std::io::{ErrorKind, Read};
use std::path::Path;

use serde_derive::{Deserialize, Serialize};
use toml::Value;

use crate::error::StapleError;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub site: Site,
    pub url: Url,
    pub pagination: Pagination,
    pub extra: HashMap<String, Value>,
}

impl Config {
    pub fn load_from_file() -> Result<Self, StapleError> {
        debug!("load config file");
        let mut file = File::open("Staple.toml")?;
        let mut string = String::new();
        file.read_to_string(&mut string)?;
        toml::from_str(&string).map_err(|e| StapleError::ConfigError(e))
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
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Url {
    pub url: String,
    pub root: String,
    pub permalink: String,
}

impl Default for Url {
    fn default() -> Self {
        Self {
            url: "localhost".to_string(),
            root: "/".to_string(),
            permalink: "{year}/{month}/{day}/{title}.html".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pagination {
    pub page_size: u32,
}

impl Default for Pagination {
    fn default() -> Self {
        Self { page_size: 10 }
    }
}
