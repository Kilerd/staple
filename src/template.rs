use std::ffi::OsStr;
use std::fs::File;
use std::io::{ErrorKind, Write};
use std::path::Path;

use tera::{compile_templates, Context, Tera};

use crate::article::Article;
use crate::config::Config;
use crate::error::StapleError;

#[derive(Debug)]
pub struct Template {
    name: String,
    tera: Tera,
}

impl Template {
    pub fn new(name: String) -> Self {
        let mut tera = compile_templates!(&format!("templates/{}/**/*", name));
        Template { name, tera }
    }

    pub fn render(self, articles: Vec<Article>, config: &Config) -> Result<(), StapleError> {
        Template::remove_folder(".render")?;
        std::fs::create_dir(".render")?;
        // index
        self.render_index()?;

        // article
        self.render_article(&articles)?;

        Template::remove_folder("public")?;
        std::fs::rename(".render", "public");

        Ok(())
    }

    pub fn remove_folder(path: &str) -> Result<(), StapleError> {
        let path1 = Path::new(path);
        if path1.exists() {
            std::fs::remove_dir_all(path1).map_err(StapleError::IoError)
        } else {
            Ok(())
        }
    }

    pub fn render_index(&self) -> Result<(), StapleError> {
        let result = self.tera.render("index.html", &Context::new())?;
        let mut result1 = File::create(".render/index.html")?;
        result1
            .write_all(result.as_bytes())
            .map_err(StapleError::IoError)
    }

    pub fn render_article(&self, articles: &Vec<Article>) -> Result<(), StapleError> {
        std::fs::create_dir(".render/articles")?;

        for article in articles {
            let mut context = Context::new();
            context.insert("article", article);
            let result = self.tera.render("article.html", &context)?;
            let mut result1 = File::create(format!(".render/articles/{}.html", article.url))?;
            result1.write_all(result.as_bytes())?;
        }
        Ok(())
    }
}
