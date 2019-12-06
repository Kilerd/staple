use crate::error::StapleError;
use serde_derive::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File, io::Read, path::Path};
use toml::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub site: Site,
    pub url: Url,
    pub pages: Option<Vec<Page>>,
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

impl Default for Config {
    fn default() -> Self {
        Config {
            site: Default::default(),
            url: Default::default(),
            pagination: Default::default(),
            pages: Default::default(),
            extra: Default::default(),
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Page {
    pub show_in_nav: bool,
    pub nav_title: String,
    pub file: String,
    pub template: String,
}

impl Default for Page {
    fn default() -> Self {
        Self {
            show_in_nav: false,
            nav_title: "".to_string(),
            file: "".to_string(),
            template: "".to_string(),
        }
    }
}
