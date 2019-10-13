use std::collections::HashMap;
use std::fs::File;
use std::io::{ErrorKind, Read};

use serde_derive::{Deserialize, Serialize};
use toml::Value;


use crate::error::StapleError;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Config {
    site: Site,
    url: Url,
    pagination: Pagination,
    extra: HashMap<String, Value>,
}

impl Config {
    pub fn load_from_file() -> Result<Self, StapleError> {
        let mut file = File::open("Staple.toml")?;
        let mut string = String::new();
        file.read_to_string(&mut string)?;
        toml::from_str(&string).map_err(|e|StapleError::ConfigError(e))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Site {
    title: String,
    subtitle: String,
    description: String,
    keywords: Vec<String>,
    author: String,
    email: String,
    utc_offset: i16,
    theme: String
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
            theme: "rubble".to_string()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Url {
    url: String,
    root: String,
    permalink: String,
}

impl Default for Url {
    fn default() -> Self {
        Self {
            url: "localhost".to_string(),
            root: "/".to_string(),
            permalink: "{year}/{month}/{day}/{title}.html".to_string()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pagination {
    page_size: u32,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page_size: 10
        }
    }
}

