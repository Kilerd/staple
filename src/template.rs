use crate::{article::Article, config::Config, error::StapleError};

use std::path::Path;
use tera::{compile_templates, Context, Tera};

#[derive(Debug)]
pub struct Template {
    name: String,
    tera: Tera,
}

impl Template {
    pub fn new(name: String) -> Self {
        let tera = compile_templates!(&format!("templates/{}/**/*", name));
        Template { name, tera }
    }

    pub fn render(self, articles: Vec<Article>, config: &Config) -> Result<(), StapleError> {
        Template::remove_folder(".render")?;
        std::fs::create_dir(".render")?;
        // index
        self.render_index(config, &articles)?;
        // article
        self.render_article(config, &articles)?;
        // pages
        self.render_pages(config)?;
        self.copy_statics_folder(config)?;

        Template::remove_folder("public")?;
        std::fs::rename(".render", "public")?;
        Ok(())
    }

    fn copy_statics_folder(&self, config: &Config) -> Result<(), StapleError> {
        let statics_folder = format!("templates/{}/statics", config.site.theme);
        if Path::new(&statics_folder).exists() {
            debug!("statics folder exist, copy to render folder");
            copy_dir::copy_dir(statics_folder, ".render/statics")?;
            //            let mut options = CopyOptions::new();
            //            options.copy_inside = true;
            //            fs_extra::dir::copy(statics_folder, ".render", &options)?;
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

    pub fn render_index(
        &self,
        config: &Config,
        articles: &Vec<Article>,
    ) -> Result<(), StapleError> {
        let mut context = Context::new();
        context.insert("config", config);
        context.insert("articles", articles);
        let result = self.tera.render("index.html", &context)?;
        std::fs::write(".render/index.html", result.as_bytes()).map_err(StapleError::IoError)
    }

    pub fn render_article(
        &self,
        config: &Config,
        articles: &Vec<Article>,
    ) -> Result<(), StapleError> {
        let article_count = articles.len();
        for (index, article) in articles.iter().enumerate() {
            println!(
                "{}/{} rendering article {}({}.md)",
                index + 1,
                article_count,
                article.title,
                article.url
            );
            let mut context = Context::new();
            context.insert("article", article);
            context.insert("config", config);
            let result = self.tera.render("article.html", &context)?;

            std::fs::create_dir(format!(".render/{}", &article.url))?;
            std::fs::write(
                format!(".render/{}/index.html", article.url),
                result.as_bytes(),
            )?;
        }
        Ok(())
    }

    pub fn render_pages(&self, config: &Config) -> Result<(), StapleError> {
        if let Some(pages) = &config.pages {
            let path = Path::new("pages");

            dbg!(&pages);
            for page in pages {
                let article = Article::load(path.join(&page.file).to_str().unwrap())?;
                let mut context = Context::new();
                context.insert("article", &article);
                context.insert("config", config);
                let result = self.tera.render(&page.template, &context)?;
                std::fs::create_dir(format!(".render/{}", &article.url))?;
                std::fs::write(
                    format!(".render/{}/index.html", article.url),
                    result.as_bytes(),
                )?;
            }
        }
        Ok(())
    }
}
