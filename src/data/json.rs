use chrono::{DateTime, FixedOffset, Utc};
use serde::{Deserialize, Serialize};

use crate::{constants::DESCRIPTION_SEPARATOR, data::MarkdownContent, error::StapleError};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonFileData {
    pub url: String,
    pub title: String,
    pub template: String,
    #[serde(default)]
    pub draw: bool,
    pub datetime: DateTime<FixedOffset>,
    pub data: HashMap<String, Value>,
    pub description: Option<MarkdownContent>,
    pub content: MarkdownContent,
}

#[doc(hidden)]
#[derive(Debug, Serialize, Deserialize)]
struct InnerData {
    pub title: String,
    pub url: String,
    pub template: String,
    #[serde(default)]
    pub draw: bool,
    pub datetime: DateTime<FixedOffset>,
    pub data: HashMap<String, Value>,
    pub content: String,
}

impl JsonFileData {
    pub fn from_str(content: &str) -> Result<Self, StapleError> {
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
            draw: data.draw,
            datetime: data.datetime,
            data: data.data,
            description,
            content: MarkdownContent::new(data.content),
        })
    }

    pub fn create(
        title: String,
        url: String,
        template: String,
        draw: bool,
    ) -> Result<(), StapleError> {
        let offset = FixedOffset::east(60 * 60 * 8);
        let data = InnerData {
            title: title.clone(),
            url,
            template,
            draw,
            datetime: Utc::now().with_timezone(&offset),
            data: HashMap::new(),
            content: "".to_string(),
        };

        let string = serde_json::to_string_pretty(&data)?;

        let file_name = title.trim().replace(" ", "-").replace("_", "-");
        let file_path = format!("data/{}.json", &file_name);
        std::fs::write(file_path, string)?;
        Ok(())
    }
}
