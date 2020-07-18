use crate::{article::Article, config::Config, error::StapleError, template::Template};
use chrono::{DateTime, FixedOffset};
use std::collections::HashMap;
use crate::article::MarkdownContent;
use serde_json::Value;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug)]
pub struct App {
    pub(crate) config: Config,
    pub(crate) template: Template,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub url:String,
    pub title: String,
    pub template: String,
    pub datetime: Option<DateTime<FixedOffset>>,
    pub data: HashMap<String, Value>,
    pub content: MarkdownContent
}

impl Data {
    pub fn from_str(content: &str) -> Result<Self, StapleError> {
        #[derive(Debug, Serialize, Deserialize)]
        struct InnerData {
            pub url:String,
            pub title: String,
            pub template: String,
            pub datetime: Option<DateTime<FixedOffset>>,
            pub data: HashMap<String, Value>,
            pub content: String
        }

        let data = serde_json::from_str::<InnerData>(content)?;
        Ok(Self {
            url: data.url,
            title: data.title,
            template: data.template,
            datetime: data.datetime,
            data: data.data,
            content: MarkdownContent::new(data.content)
        })
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub enum FileData {
    JsonFile(Data),
    MarkdownFile(Article)
}

impl App {
    pub fn load() -> Result<Self, StapleError> {
        let config = Config::load_from_file()?;
        debug!("init template");
        let template = Template::new(config.get_theme()?);
        Ok(Self { config, template })
    }
    pub fn render(self) -> Result<(), StapleError> {
        // debug!("starting render");
        // debug!("load all article");
        // let articles = Article::load_all_article();
        // self.template.render(articles?, &self.config)
        let vec = self.load_all_data()?;
        self.template.render2(vec, &self.config)

    }

    pub fn load_all_data(&self) -> Result<Vec<FileData>, StapleError> {
        let path = Path::new("data");
        let mut articles = vec![];
        let dir = path.read_dir()?;

        for path in dir {
            if let Ok(p) = path {
                let file_path = p.path();
                if file_path.is_file(){
                    let option = file_path.extension().and_then(|e| e.to_str()).unwrap_or("");

                    match option {
                        "md" => {
                            let result2 = Article::load(file_path.to_str().unwrap())?;
                            articles.push(FileData::MarkdownFile(result2));
                        },
                        "json" => {
                            let result = std::fs::read_to_string(file_path)?;
                            let data = Data::from_str(&result)?;
                            articles.push(FileData::JsonFile(data));
                        }
                        _ => {}
                    }
                }
            }
        }
        // articles.sort_by(|one, other| other.meta.date.cmp(&one.meta.date));
        Ok(articles)
    }
}
