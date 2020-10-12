use chrono::{DateTime, FixedOffset, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    constants::DESCRIPTION_SEPARATOR,
    data::{
        types::{CreationOptions, FileType},
        MarkdownContent, PageInfo,
    },
    error::StapleError,
};
use serde_json::Value;
use std::{collections::HashMap, path::Path};

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonFileData {
    pub path: String,
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

impl FileType for JsonFileData {
    type Output = JsonFileData;

    fn load(file: impl AsRef<Path>) -> Result<Self::Output, StapleError> {
        let file = file.as_ref();
        let data_file_content = std::fs::read_to_string(file)?;

        let data = serde_json::from_str::<InnerData>(&data_file_content)?;
        let description = if data.content.contains(DESCRIPTION_SEPARATOR) {
            let content_split: Vec<&str> = data.content.splitn(2, DESCRIPTION_SEPARATOR).collect();
            Some(MarkdownContent::new(content_split[0].to_string()))
        } else {
            None
        };
        Ok(Self {
            path: file.to_str().unwrap().to_string(),
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

    fn create(_file: impl AsRef<Path>, options: &CreationOptions) -> Result<(), StapleError> {
        let offset = FixedOffset::east(60 * 60 * 8);
        let data = InnerData {
            title: options.title.clone(),
            url: options.url.clone(),
            template: options.template.clone(),
            draw: options.draw,
            datetime: Utc::now().with_timezone(&offset),
            data: HashMap::new(),
            content: "".to_string(),
        };

        let string = serde_json::to_string_pretty(&data)?;

        let file_name = options
            .title
            .clone()
            .trim()
            .replace(" ", "-")
            .replace("_", "-");
        let output_path = _file
            .as_ref()
            .join("data")
            .join(format!("{}.json", &file_name));
        std::fs::write(output_path, string)?;
        Ok(())
    }

    fn into_page_info(self) -> PageInfo {
        PageInfo {
            file: self.path,
            url: self.url,
            title: self.title,
            template: self.template,
            draw: self.draw,
            datetime: self.datetime,
            data: self.data,
            description: self.description,
        }
    }
}
