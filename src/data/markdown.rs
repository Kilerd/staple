use std::{collections::HashMap, path::Path};
use std::fs::File;
use std::io::Write;

use chrono::{DateTime, FixedOffset, Local, Utc};
use itertools::Itertools;
use pest::Parser;
use pulldown_cmark::{Event, Options};
use regex::Regex;
use serde_derive::{Deserialize, Serialize};

use crate::constants::DESCRIPTION_SEPARATOR;
use crate::constants::LINE_ENDING;
use crate::data::MarkdownContent;
use crate::error::StapleError;


#[derive(Parser)]
#[grammar = "data/article.pest"] // relative to src
struct ArticleParser;


#[derive(Serialize, Deserialize, Debug)]
pub struct ArticleMeta {
    pub title: String,
    pub url: String,
    pub tags: Vec<String>,
    pub date: DateTime<FixedOffset>,
    pub extra: HashMap<String, String>,
    pub description: Option<MarkdownContent>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MarkdownFileData {
    pub url: String,
    pub title: String,
    pub template: String,
    pub datetime: DateTime<FixedOffset>,
    #[serde(default)]
    pub draw: bool,
    // todo support multiple layers of data
    pub data: HashMap<String, String>,
    pub content: MarkdownContent,
    pub description: Option<MarkdownContent>,
}


impl MarkdownFileData {
    pub fn load_all_article() -> Result<Vec<MarkdownFileData>, StapleError> {
        let path = Path::new("data");
        let mut articles = vec![];
        let dir = path.read_dir()?;

        for path in dir {
            if let Ok(p) = path {
                let file_path = p.path();
                let is_md_file =
                    file_path.extension().map(|extension| extension.eq("md")) == Some(true);
                if is_md_file && file_path.is_file() {
                    articles.push(MarkdownFileData::load(file_path.to_str().unwrap())?)
                }
            }
        }
        // articles.sort_by(|one, other| other.meta.date.cmp(&one.meta.date));
        Ok(articles)
    }

    pub fn load(file: &str) -> Result<MarkdownFileData, StapleError> {
        debug!("load article {}", &file);
        let string = std::fs::read_to_string(file)?;
        let mut metas: HashMap<String, String> = HashMap::new();
        let mut content = String::new();

        let parse1 = ArticleParser::parse(Rule::article, &string);
        let x = parse1.expect("").next().unwrap();
        for pair in x.into_inner() {
            match pair.as_rule() {
                Rule::meta => {
                    for meta in pair.into_inner() {
                        let mut x1 = meta.into_inner();
                        let key: String = x1.next().unwrap().as_str().to_string();
                        let value: String = x1.next().unwrap().as_str().to_string();
                        metas.insert(key.to_lowercase(), value);
                    }
                }
                Rule::content => {
                    content.push_str(pair.as_str());
                }
                Rule::EOI => (),
                _ => unreachable!(),
            }
        }

        let url = metas.remove("url").ok_or(StapleError::ArticleError {
            filename: file.to_string(),
            reason: "url does not exist in article's metadata".to_string(),
        })?;
        let title = metas.remove("title").ok_or(StapleError::ArticleError {
            filename: file.to_string(),
            reason: "title does not exist in article's metadata".to_string(),
        })?;
        let template = metas.remove("template").ok_or(StapleError::ArticleError {
            filename: file.to_string(),
            reason: "template does not exist in article's metadata".to_string(),
        })?;

        let draw = metas.remove("draw").map(|value| value.to_lowercase().eq("true")).unwrap_or(false);

        let option_date = metas
            .remove("datetime")
            .ok_or(StapleError::ArticleError {
                filename: file.to_string(),
                reason: "datetime does not exist in article's metadata".to_string(),
            })
            .map(|raw| DateTime::parse_from_rfc3339(&raw))?
            .map_err(|e| StapleError::ArticleError {
                filename: file.to_string(),
                reason: format!("parse date error {}", e),
            })?;

        let description = if content.contains(DESCRIPTION_SEPARATOR) {
            let content_split: Vec<&str> = content.splitn(2, DESCRIPTION_SEPARATOR).collect();
            Some(MarkdownContent::new(content_split[0].to_string()))
        } else {
            None
        };

        Ok(MarkdownFileData {
            url,
            title,
            template,
            datetime: option_date,
            description,
            content: MarkdownContent::new(content),
            data: metas,
            draw,
        })
    }

    pub fn create(title: String, url: String, template: String, draw: bool) -> Result<(), StapleError> {
        let offset = FixedOffset::east(60 * 60 * 8);
        let datetime = Utc::now().with_timezone(&offset).to_rfc3339();

        let mut content = String::new();

        content.push_str(&format!(" - title = {}{}", &title, LINE_ENDING));
        content.push_str(&format!(" - url = {}{}", &url, LINE_ENDING));
        content.push_str(&format!(" - datetime = {}{}", datetime, LINE_ENDING));
        content.push_str(&format!(" - template = {}{}", template, LINE_ENDING));
        content.push_str(&format!(" - draw = {}{}", draw, LINE_ENDING));
        content.push_str(LINE_ENDING);

        let file_name = title.trim().replace(" ", "-").replace("_", "-");
        let file_path = format!("data/{}.md", &file_name);
        std::fs::write(file_path, content)?;
        Ok(())
    }
}


