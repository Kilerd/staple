use crate::error::StapleError;
use chrono::{DateTime, FixedOffset};
use pest::Parser;
use serde_derive::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path};

#[derive(Parser)]
#[grammar = "article.pest"] // relative to src
struct ArticleParser;

#[derive(Serialize, Deserialize, Debug)]
pub struct Article {
    pub title: String,
    pub url: String,
    pub tags: Vec<String>,
    pub date: DateTime<FixedOffset>,
    pub raw_content: String,
    pub markdown: String,
    pub extra: HashMap<String, String>,
}

impl Article {
    pub fn load_all_article() -> Result<Vec<Article>, StapleError> {
        let path = Path::new("articles");
        let mut articles = vec![];

        for path in path.read_dir()? {
            if let Ok(p) = path {
                articles.push(Article::load(p.path().to_str().unwrap())?)
            }
        }
        articles.sort_by(|one, other| other.date.cmp(&one.date));
        Ok(articles)
    }

    pub fn load(file: &str) -> Result<Article, StapleError> {
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

        let title = metas.remove("title").ok_or(StapleError::ArticleError {
            filename: file.to_string(),
            reason: "title does not exist in article's metadata".to_string(),
        })?;
        let url = metas.remove("url").ok_or(StapleError::ArticleError {
            filename: file.to_string(),
            reason: "url does not exist in article's metadata".to_string(),
        })?;
        let tags: Vec<String> = metas
            .remove("tags")
            .map(|raw| raw.split(",").map(|e| e.trim().to_string()).collect())
            .ok_or(StapleError::ArticleError {
                filename: file.to_string(),
                reason: "tags does not exist in article's metadata".to_string(),
            })?;
        let option_date = metas
            .remove("datetime")
            .ok_or(StapleError::ArticleError {
                filename: file.to_string(),
                reason: "datetime does not exist in article's metadata".to_string(),
            })
            .map(|raw| DateTime::parse_from_str(&raw, "%Y-%m-%d %H:%M:%S %z"))?
            .map_err(|e| StapleError::ArticleError {
                filename: file.to_string(),
                reason: format!("parse date error {}", e),
            })?;

        Ok(Article {
            title: title.to_string(),
            url: url.to_string(),
            tags,
            date: option_date,
            extra: metas,
            raw_content: content,
            markdown: "".to_string(),
        })
    }
}
