use std::collections::HashMap;
use std::fs::File;
use std::io::{ErrorKind, Read};

use serde_derive::{Deserialize, Serialize};
use toml::Value;


use crate::error::StapleError;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Config {
    base: Base,
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
pub struct Base {
    title: String,
    description: String,
    url: String,
    author: String,
    email: String,
}