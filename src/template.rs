use crate::{article::Article, config::Config, error::StapleError};

use rss::{Channel, ChannelBuilder, Item, ItemBuilder};
use std::collections::HashMap;
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
        // rss
        self.render_rss(config, &articles)?;
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
        dbg!(&config);
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
                article.meta.title,
                article.meta.url
            );
            let mut context = Context::new();
            context.insert("article", article);
            context.insert("config", config);
            let result = self.tera.render("article.html", &context)?;

            std::fs::create_dir(format!(".render/{}", &article.meta.url))?;
            std::fs::write(
                format!(".render/{}/index.html", article.meta.url),
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
                std::fs::create_dir(format!(".render/{}", &article.meta.url))?;
                std::fs::write(
                    format!(".render/{}/index.html", article.meta.url),
                    result.as_bytes(),
                )?;
            }
        }
        Ok(())
    }
    pub fn render_rss(&self, config: &Config, articles: &Vec<Article>) -> Result<(), StapleError> {
        let url1 = url::Url::parse(&config.url.url)?;
        let result = url1.join(&config.url.root)?;

        let items: Vec<Item> = articles
            .into_iter()
            .take(10)
            .map(|item| {
                let item_url = result.join(&item.meta.url).unwrap().to_string();
                ItemBuilder::default()
                    .title(item.meta.title.clone())
                    .link(item_url)
                    .description(
                        item.meta
                            .description
                            .as_ref()
                            .map(|description| description.html.clone())
                            .unwrap_or_default(),
                    )
                    .content(item.content.html.clone())
                    .pub_date(item.meta.date.to_string())
                    .build()
                    .unwrap()
            })
            .collect();

        let mut namespaces: HashMap<String, String> = HashMap::new();
        namespaces.insert(
            "dc".to_string(),
            "http://purl.org/dc/elements/1.1/".to_string(),
        );
        namespaces.insert(
            "content".to_string(),
            "http://purl.org/rss/1.0/modules/content/".to_string(),
        );
        namespaces.insert(
            "atom".to_string(),
            "http://www.w3.org/2005/Atom".to_string(),
        );
        namespaces.insert(
            "media".to_string(),
            "http://search.yahoo.com/mrss/".to_string(),
        );

        let channel: Channel = ChannelBuilder::default()
            .title(config.site.title.clone())
            .description(config.site.description.clone())
            .generator("Staple".to_string())
            .link(result.to_string())
            .items(items)
            .namespaces(namespaces)
            .build()
            .unwrap();

        std::fs::write(".render/rss.xml", channel.to_string().as_bytes())?;
        Ok(())
    }
}
