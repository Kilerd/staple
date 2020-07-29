use serde::{Serialize, Deserialize};
use chrono::{DateTime, FixedOffset};

use std::collections::HashMap;
use serde_json::Value;
use crate::error::StapleError;
use crate::data::MarkdownContent;
use crate::constants::DESCRIPTION_SEPARATOR;

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonFileData {
    pub url: String,
    pub title: String,
    pub template: String,
    pub datetime: DateTime<FixedOffset>,
    pub data: HashMap<String, Value>,
    pub description: Option<MarkdownContent>,
    pub content: MarkdownContent,
}

impl JsonFileData {
    pub fn from_str(content: &str) -> Result<Self, StapleError> {
        #[derive(Debug, Serialize, Deserialize)]
        struct InnerData {
            pub url: String,
            pub title: String,
            pub template: String,
            pub datetime: DateTime<FixedOffset>,
            pub data: HashMap<String, Value>,
            pub content: String,
        }

        let data = serde_json::from_str::<InnerData>(content)?;
        let description = if data.content.contains(DESCRIPTION_SEPARATOR) {
            let content_split: Vec<&str> = content.splitn(2, DESCRIPTION_SEPARATOR).collect();
            Some(MarkdownContent::new(content_split[0].to_string()))
        } else {
            None
        };
        Ok(Self {
            url: data.url,
            title: data.title,
            template: data.template,
            datetime: data.datetime,
            data: data.data,
            description,
            content: MarkdownContent::new(data.content),
        })
    }
}