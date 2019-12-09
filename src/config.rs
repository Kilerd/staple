use crate::article::{Article, ArticleMeta};
use crate::error::StapleError;
use serde_derive::{Deserialize, Serialize};
use std::ops::Deref;
use std::{collections::HashMap, fs::File, io::Read, path::Path};
use toml::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct PageMeta {
    pub meta: ArticleMeta,
    pub nav_title: String,
    pub file: String,
    pub template: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub site: Site,
    pub url: Url,
    pub pages: Option<Vec<PageMeta>>,
    pub pagination: Pagination,
    pub rss: RssConfig,
    pub extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigFile {
    pub site: Site,
    pub url: Url,
    pub pages: Option<Vec<Page>>,
    pub pagination: Pagination,
    pub rss: RssConfig,
    pub extra: HashMap<String, Value>,
}

impl Config {
    pub fn load_from_file() -> Result<Self, StapleError> {
        debug!("load config file");
        let mut file = File::open("Staple.toml")?;
        let mut string = String::new();
        file.read_to_string(&mut string)?;
        let result: ConfigFile = toml::from_str(&string)?;
        Config::new_from_file(result)
    }

    pub fn new_from_file(config_file: ConfigFile) -> Result<Self, StapleError> {
        let page_metas = if let Some(pages) = config_file.pages {
            let mut page_metas = vec![];
            let path = Path::new("pages");
            for page in pages {
                let article = Article::load(path.join(&page.file).to_str().unwrap())?;

                let page_meta = PageMeta {
                    meta: article.meta,
                    nav_title: page.nav_title,
                    file: page.file,
                    template: page.template,
                };
                page_metas.push(page_meta);
            }
            Some(page_metas)
        } else {
            None
        };

        Ok(Self {
            site: config_file.site,
            url: config_file.url,
            pages: page_metas,
            pagination: config_file.pagination,
            rss: config_file.rss,
            extra: config_file.extra,
        })
    }

    pub fn get_theme(&self) -> Result<String, StapleError> {
        let empty_theme = self.site.theme.eq("");
        let theme_exist = !Path::new("templates")
            .join(self.site.theme.clone())
            .exists();
        if empty_theme || theme_exist {
            Err(StapleError::ThemeNotFound(self.site.theme.clone()))
        } else {
            Ok(self.site.theme.clone())
        }
    }

    pub fn get_default_file() -> ConfigFile {
        ConfigFile::default()
    }
}

impl Default for ConfigFile {
    fn default() -> Self {
        ConfigFile {
            site: Default::default(),
            url: Default::default(),
            pagination: Default::default(),
            pages: Default::default(),
            extra: Default::default(),
            rss: Default::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Site {
    pub title: String,
    pub subtitle: String,
    pub description: String,
    pub keywords: Vec<String>,
    pub author: String,
    pub email: String,
    pub utc_offset: i16,
    pub theme: String,
}

impl Default for Site {
    fn default() -> Self {
        Self {
            title: "".to_string(),
            subtitle: "".to_string(),
            description: "".to_string(),
            keywords: vec![],
            author: "".to_string(),
            email: "".to_string(),
            utc_offset: 800,
            theme: "rubble".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Url {
    pub url: String,
    pub root: String,
}

impl Default for Url {
    fn default() -> Self {
        Self {
            url: "http://localhost:8000".to_string(),
            root: "/".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pagination {
    pub page_size: u32,
}

impl Default for Pagination {
    fn default() -> Self {
        Self { page_size: 10 }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Page {
    pub show_in_nav: bool,
    pub nav_title: String,
    pub file: String,
    pub template: String,
}

impl Default for Page {
    fn default() -> Self {
        Self {
            show_in_nav: false,
            nav_title: "".to_string(),
            file: "".to_string(),
            template: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RssConfig {
    pub enable: bool,
    pub article_limited: usize,
}

impl Default for RssConfig {
    fn default() -> Self {
        Self {
            enable: true,
            article_limited: 10,
        }
    }
}

pub struct ConfigView {
    config: Config,
    page_meta: Option<Vec<ArticleMeta>>,
}

impl Deref for ConfigView {
    type Target = Config;

    fn deref(&self) -> &Self::Target {
        &self.config
    }
}
