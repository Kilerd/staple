use crate::{config::Config, error::StapleError, template::Template};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use std::path::Path;
use crate::data::{DataFile, JsonFileData, MarkdownFileData};

#[derive(Debug)]
pub struct App {
    pub(crate) config: Config,
    pub(crate) template: Template,
}


impl App {
    pub fn load() -> Result<Self, StapleError> {
        let config = Config::load_from_file()?;
        debug!("init template");
        let template = Template::new(config.get_theme()?);
        Ok(Self { config, template })
    }

    pub fn render(self) -> Result<(), StapleError> {
        let vec = self.load_all_data()?
            .into_iter()
            .filter(|article| !article.is_draw())
            .collect();
        self.template.render(vec, &self.config)
    }

    pub fn load_all_data(&self) -> Result<Vec<DataFile>, StapleError> {
        let path = Path::new("data");
        let mut articles = vec![];
        let dir = path.read_dir()?;

        for path in dir {
            if let Ok(p) = path {
                let file_path = p.path();
                if file_path.is_file() {
                    let option = file_path.extension().and_then(|e| e.to_str()).unwrap_or("");

                    match option {
                        "md" => {
                            let result2 = MarkdownFileData::load(file_path.to_str().unwrap())?;
                            articles.push(DataFile::MarkdownFile(result2));
                        }
                        "json" => {
                            let result = std::fs::read_to_string(file_path)?;
                            let data = JsonFileData::from_str(&result)?;
                            articles.push(DataFile::JsonFile(data));
                        }
                        _ => {}
                    }
                }
            }
        }
        articles.sort_by(|one, other| one.get_created_time().cmp(other.get_created_time()));
        Ok(articles)
    }
}
