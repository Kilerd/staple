use chrono::{DateTime, FixedOffset};
use itertools::Itertools;
use pulldown_cmark::Event;
use pulldown_cmark::Options;
use regex::Regex;
use serde::{Deserialize, Serialize};

pub use crate::data::json::JsonFileData;
pub use crate::data::markdown::MarkdownFileData;
use std::collections::HashMap;
use serde_json::Value;
use std::path::Path;
use crate::error::StapleError;

mod json;
mod markdown;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Either<A, B> {
    Left(A),
    Right(B),
}


#[derive(Serialize, Deserialize, Debug)]
pub struct MarkdownContent {
    pub markdown: String,
    pub html: String,
}

impl MarkdownContent {
    pub fn new(raw: String) -> Self {
        let mut html_output = String::new();
        let options = Options::all();
        let parser = pulldown_cmark::Parser::new_ext(&raw, options).flat_map(|event| match event {
            Event::Text(text) => {
                let mut text_chars = text.as_bytes().into_iter();
                let mut events = vec![];
                let re = Regex::new(r#"\{(?P<title>[^}]+)\}\((?P<ruby>[^\)]+)\)"#).unwrap();
                let mut last_end_index = 0;
                for captures in re.captures_iter(&text) {
                    let ruby_group = captures.get(0).unwrap();
                    let ruby_name = captures.name("title").unwrap().as_str().to_string();
                    let ruby_description = captures.name("ruby").unwrap().as_str().to_string();
                    let ruby_group_start = ruby_group.start();

                    if last_end_index != ruby_group_start {
                        let ruby_prefix_content: Vec<u8> = text_chars
                            .by_ref()
                            .take(ruby_group_start - last_end_index)
                            .map(|i| *i)
                            .collect();
                        let string = String::from_utf8(ruby_prefix_content).unwrap();
                        events.push(Event::Text(string.into()));
                    }
                    last_end_index = ruby_group.end();
                    text_chars = text_chars.dropping(ruby_group.end() - ruby_group.start());

                    events.push(Event::Html("<ruby>".into()));
                    events.push(Event::Text(ruby_name.into()));
                    events.push(Event::Html("<rp>(</rp><rt>".into()));
                    events.push(Event::Text(ruby_description.into()));
                    events.push(Event::Html("</rt><rp>)</rp>".into()));
                    events.push(Event::Html("</ruby>".into()));
                }
                if last_end_index < text.len() {
                    let rest: Vec<u8> = text_chars.map(|i| *i).collect();
                    let rest = String::from_utf8(rest).unwrap();
                    events.push(Event::Text(rest.into()));
                }
                events
            }

            _ => vec![event],
        });
        pulldown_cmark::html::push_html(&mut html_output, parser);

        Self {
            markdown: raw,
            html: html_output,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DataFile {
    JsonFile(JsonFileData),
    MarkdownFile(MarkdownFileData),
}

impl DataFile {
    pub fn template(&self) -> &str {
        match &self {
            DataFile::JsonFile(data) => &data.template,
            DataFile::MarkdownFile(data) => &data.template,
        }
    }

    pub fn url(&self) -> String {
        let url = match &self {
            DataFile::JsonFile(data) => &data.url,
            DataFile::MarkdownFile(data) => &data.url,
        };

        if url.starts_with("/") {
            url.clone()
        } else {
            format!("/{}", &url)
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PageInfo {
    pub file: String,
    pub url: String,
    pub title: String,
    pub template: String,
    #[serde(default)]
    pub draw: bool,
    pub datetime: DateTime<FixedOffset>,
    pub data: Either<HashMap<String, Value>, HashMap<String, String>>,
    pub description: Option<MarkdownContent>,
}

impl PageInfo {
    pub fn to_full_article(&self) -> Result<DataFile, StapleError> {
        let path = Path::new(&self.file);
        let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        match extension {
            "md" => {
                MarkdownFileData::load(path.to_str().unwrap()).map(DataFile::MarkdownFile)
            }
            "json" => {
                let result = std::fs::read_to_string(path)?;
                JsonFileData::from_str(&result).map(DataFile::JsonFile)
            }
            _ => { unreachable!() }
        }
    }
}


#[cfg(test)]
mod test {
    use crate::data::MarkdownContent;

    #[test]
    fn should_render_ruby_tag() {
        let content =
            MarkdownContent::new("**this** is **{ruby}(ruby description) aaa** tag".to_string());
        assert_eq!(
            "<p><strong>this</strong> is <strong><ruby>ruby<rp>(</rp><rt>ruby description</rt><rp>)</rp></ruby> aaa</strong> tag</p>\n",
            content.html
        );
    }
}
