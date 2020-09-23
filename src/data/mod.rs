use chrono::{DateTime, FixedOffset};
use itertools::Itertools;
use pulldown_cmark::{Event, Options};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use std::{collections::HashMap, path::Path};

use crate::{
    data::types::{json::JsonFileData, markdown::MarkdownFileData, FileType},
    error::StapleError,
};

pub(crate) mod types;

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
                let mut text_chars = text.as_bytes().iter();
                let mut events = vec![];
                let re =
                    Regex::new(r#"\{(?P<title>[^}]+)}\((?P<ruby>[^)]+)\)"#).expect("invalid regex");
                let mut last_end_index = 0;
                for captures in re.captures_iter(&text) {
                    let ruby_group = captures.get(0).unwrap();
                    let ruby_name = captures
                        .name("title")
                        .expect("invalid title")
                        .as_str()
                        .to_string();
                    let ruby_description = captures
                        .name("ruby")
                        .expect("invalid ruby")
                        .as_str()
                        .to_string();
                    let ruby_group_start = ruby_group.start();

                    if last_end_index != ruby_group_start {
                        let ruby_prefix_content: Vec<u8> = text_chars
                            .by_ref()
                            .take(ruby_group_start - last_end_index)
                            .copied()
                            .collect();
                        let string =
                            String::from_utf8(ruby_prefix_content).expect("invalid utf8 string");
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
                    let rest: Vec<u8> = text_chars.copied().collect();
                    let rest = String::from_utf8(rest).expect("invalid utf8 string");
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
#[serde(tag = "types")]
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

    pub fn load(path: impl AsRef<Path>) -> Result<Option<PageInfo>, StapleError> {
        let extension = path.as_ref().extension().and_then(|e| e.to_str());
        match extension {
            Some("md") => {
                MarkdownFileData::load(path.as_ref()).map(|full| Some(full.into_page_info()))
            }
            Some("json") => {
                JsonFileData::load(path.as_ref()).map(|full| Some(full.into_page_info()))
            }
            _ => Ok(None),
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
    pub data: HashMap<String, Value>,
    pub description: Option<MarkdownContent>,
}

impl PageInfo {
    pub fn to_full_article(&self) -> Result<DataFile, StapleError> {
        let path = Path::new(&self.file);
        let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        match extension {
            "md" => MarkdownFileData::load(path.to_str().expect("invalid file path encoding"))
                .map(DataFile::MarkdownFile),

            "json" => {
                let result = std::fs::read_to_string(path)?;
                JsonFileData::load(&result).map(DataFile::JsonFile)
            }
            _ => unreachable!(),
        }
    }

    pub fn output_file_name(&self) -> String {
        let url = Path::new(&self.url);
        let has_extension = url.extension().is_some();
        let start_with_slash = self.url.starts_with('/');
        let end_with_slash = self.url.ends_with('/');

        format!(
            "{}{}{}",
            if start_with_slash { "" } else { "/" },
            &self.url,
            if has_extension {
                ""
            } else if end_with_slash {
                "index.html"
            } else {
                "/index.html"
            }
        )
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
    #[test]
    fn should_render_ruby_tag_with_utf8() {
        let content = MarkdownContent::new("这是一个{RUBY带中文}(中文下标)标签".to_string());
        assert_eq!(
            "<p>这是一个<ruby>RUBY带中文<rp>(</rp><rt>中文下标</rt><rp>)</rp></ruby>标签</p>\n",
            content.html
        );
    }
}
