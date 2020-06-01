use crate::error::StapleError;
use chrono::{DateTime, FixedOffset, Local};

use crate::constants::LINE_ENDING;
use itertools::Itertools;
use pest::Parser;
use pulldown_cmark::{Event, Options};
use regex::Regex;
use serde_derive::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::{collections::HashMap, path::Path};

#[derive(Parser)]
#[grammar = "article.pest"] // relative to src
struct ArticleParser;

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
                let mut text_chars = text.chars();
                let mut events = vec![];
                let re = Regex::new(r#"\{(?P<title>.+)\}\((?P<ruby>.+)\)"#).unwrap();
                let mut last_end_index = 0;
                for captures in re.captures_iter(&text) {
                    let ruby_group = captures.get(0).unwrap();
                    let ruby_name = captures.name("title").unwrap().as_str().to_string();
                    let ruby_description = captures.name("ruby").unwrap().as_str().to_string();
                    let ruby_group_start = ruby_group.start();

                    if last_end_index != ruby_group_start {
                        let ruby_prefix_content: String = text_chars
                            .by_ref()
                            .take(ruby_group_start - last_end_index)
                            .collect();
                        events.push(Event::Text(ruby_prefix_content.into()));
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
                    let rest: String = text_chars.collect();
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
pub struct Article {
    pub meta: ArticleMeta,
    pub content: MarkdownContent,
}

impl Article {
    pub fn load_all_article() -> Result<Vec<Article>, StapleError> {
        let path = Path::new("articles");
        let mut articles = vec![];
        let dir = path.read_dir()?;

        for path in dir {
            if let Ok(p) = path {
                let file_path = p.path();
                let is_md_file =
                    file_path.extension().map(|extension| extension.eq("md")) == Some(true);
                if is_md_file && file_path.is_file() {
                    articles.push(Article::load(file_path.to_str().unwrap())?)
                }
            }
        }
        articles.sort_by(|one, other| other.meta.date.cmp(&one.meta.date));
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
            .unwrap_or_default();
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

        let description = if content.contains("<!--more-->") {
            let content_split: Vec<&str> = content.splitn(2, "<!--more-->").collect();
            Some(MarkdownContent::new(content_split[0].to_string()))
        } else {
            None
        };

        Ok(Article {
            meta: ArticleMeta {
                title: title.to_string(),
                url: url.to_string(),
                tags,
                date: option_date,
                extra: metas,
                description,
            },
            content: MarkdownContent::new(content),
        })
    }

    pub fn new_template(
        url: &str,
        title: &Option<String>,
        tags: &Vec<String>,
    ) -> Result<(), StapleError> {
        let url = url.to_string();
        let url_for_text = url.replace(" ", "-");

        let tags = tags.join(", ");

        let title = title.as_ref().unwrap_or(&url);
        let path = Path::new("articles");
        let mut result = File::create(path.join(format!("{}.md", url_for_text)))?;
        result.write(&format!(" - title = {}{}", title, LINE_ENDING).as_bytes())?;
        result.write(&format!(" - url = {}{}", &url_for_text, LINE_ENDING).as_bytes())?;
        result.write(&format!(" - tags = {}{}", tags, LINE_ENDING).as_bytes())?;
        let dt: DateTime<Local> = Local::now();
        let format1 = dt.format("%Y-%m-%d %H:%M:%S %z").to_string();
        result.write(&format!(" - datetime = {}{}", format1, LINE_ENDING).as_bytes())?;
        result.write(LINE_ENDING.as_bytes())?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::article::MarkdownContent;

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
