use crate::{config::Config, error::StapleError};

use std::path::Path;
use tera::{compile_templates, Tera};

use serde::Serialize;

use crate::constants::LIVE_RELOAD_CODE;
use crate::data::{DataFile, PageInfo};

#[derive(Debug, Serialize)]
pub struct DevelopData {
    live_reload: &'static str,
}

impl DevelopData {
    pub fn new(is_develop: bool) -> Self {
        let live_reload = if is_develop { LIVE_RELOAD_CODE } else { "" };
        DevelopData { live_reload }
    }
}

#[derive(Debug, Serialize)]
pub struct RenderData<'a> {
    page: DataFile,
    config: &'a Config,
    develop: &'a DevelopData,
    pages: &'a Vec<PageInfo>,
}

impl<'a> RenderData<'a> {
    pub fn new(page: DataFile, pages: &'a Vec<PageInfo>, config: &'a Config, develop: &'a DevelopData) -> Self {
        RenderData {
            page,
            pages,
            config,
            develop,
        }
    }
}

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

    pub fn render(
        self,
        articles: Vec<PageInfo>,
        config: &Config,
        is_develop_mode: bool,
    ) -> Result<(), StapleError> {
        Template::remove_folder(".render")?;
        std::fs::create_dir(".render")?;

        // todo can be parallel rendering
        for article in articles.iter() {
            self.render_article(config, article, &articles, is_develop_mode)?;
        }

        self.copy_statics_folder(config)?;
        self.copy_statics(config)?;

        Template::remove_folder("public")?;
        std::fs::rename(".render", "public")?;
        Ok(())
    }

    pub fn render_article(
        &self,
        config: &Config,
        article: &PageInfo,
        articles: &Vec<PageInfo>,
        is_develop_mode: bool,
    ) -> Result<(), StapleError> {
        let debug_data = DevelopData::new(is_develop_mode);

        let full_article = article.to_full_article()?;

        let data = RenderData::new(full_article, articles, config, &debug_data);
        let result = self.tera.render(data.page.template(), &data)?;
        let url = data.page.url();
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
            info!("coping statics from {} to {}", &statics.from, &statics.to);
            std::fs::copy(&statics.from, buf)?;
        }
        Ok(())
    }
}
