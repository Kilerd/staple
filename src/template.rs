use crate::{config::Config, error::StapleError};

use rss::{Channel, ChannelBuilder, Guid, Item, ItemBuilder};
use std::collections::HashMap;
use std::path::Path;
use tera::{compile_templates, Context, Tera};
use toml::ser::SerializeTable::Table;
use toml::Value;

use crate::data::{DataFile, JsonFileData, MarkdownFileData};

#[derive(Debug)]
pub struct Template {
    name: String,
    tera: Tera,
}

impl Template {
    pub fn new(name: String) -> Self {
        let tera = compile_templates!(&format!("templates/{}/**/*.html", name));
        Template { name, tera }
    }

    pub fn render(self, articles: Vec<DataFile>, config: &Config) -> Result<(), StapleError> {
        Template::remove_folder(".render")?;
        std::fs::create_dir(".render")?;

        for article in articles {
            match article {
                DataFile::JsonFile(data) => {
                    self.render_json(config, data)?
                }
                DataFile::MarkdownFile(data) => {
                    self.render_markdown(config, data)?
                }
            }
        }

        self.copy_statics_folder(config)?;
        self.copy_statics(config)?;

        Template::remove_folder("public")?;
        std::fs::rename(".render", "public")?;
        Ok(())
    }

    pub fn render_json(
        &self,
        config: &Config,
        article: JsonFileData,
    ) -> Result<(), StapleError> {
        let template = &article.template;
        let url = if article.url.starts_with("/") {
            article.url.clone()
        } else {
            format!("/{}", &article.url)
        };

        let mut context = Context::new();
        context.insert("page", &article);
        context.insert("config", config);
        let result = self.tera.render(&template, &context)?;
        let string = format!(".render{}/index.html", url);
        let path = Path::new(&string).parent();
        if let Some(p) = path {
            if !p.exists() {
                std::fs::create_dir_all(p)?;
            }
        }
        std::fs::write(string, result.as_bytes()).map_err(StapleError::IoError)
    }

    pub fn render_markdown(
        &self,
        config: &Config,
        article: MarkdownFileData,
    ) -> Result<(), StapleError> {
        let template = &article.template;
        let url = if article.url.starts_with("/") {
            article.url.clone()
        } else {
            format!("/{}", &article.url)
        };

        let mut context = Context::new();
        context.insert("page", &article);
        context.insert("config", config);
        let result = self.tera.render(&template, &context)?;
        let string = format!(".render{}/index.html", url);

        let path = Path::new(&string).parent();
        if let Some(p) = path {
            if !p.exists() {
                std::fs::create_dir_all(p)?;
            }
        }
        std::fs::write(string, result.as_bytes()).map_err(StapleError::IoError)
    }


    fn copy_statics_folder(&self, config: &Config) -> Result<(), StapleError> {
        let statics_folder = format!("templates/{}/statics", config.site.theme);
        if Path::new(&statics_folder).exists() {
            debug!("statics folder exist, copy to render folder");
            copy_dir::copy_dir(statics_folder, ".render/statics")?;
        }
        Ok(())
    }

    pub fn remove_folder(path: &str) -> Result<(), StapleError> {
        let path1 = Path::new(path);
        if path1.exists() {
            debug!("remove folder {}", path);
            std::fs::remove_dir_all(path1).map_err(StapleError::IoError)
        } else {
            Ok(())
        }
    }


    pub fn copy_statics(&self, config: &Config) -> Result<(), StapleError> {
        let path1 = Path::new(".render");
        for statics in &config.statics {
            let buf = path1.join(&statics.to);
            println!("coping statics from {} to {}", &statics.from, &statics.to);
            std::fs::copy(&statics.from, buf)?;
        }
        Ok(())
    }
}
