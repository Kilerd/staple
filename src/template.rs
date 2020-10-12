use crate::{config::Config, error::StapleError};

use std::path::{Path, PathBuf};
use tera::{Context, Tera};

use serde::Serialize;

use crate::{
    constants::{LIVE_RELOAD_CODE, PUBLIC_FOLDER, RENDER_FOLDER},
    data::{DataFile, PageInfo},
};

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
    pages: &'a [PageInfo],
}

impl<'a> RenderData<'a> {
    pub fn new(
        page: DataFile,
        pages: &'a [PageInfo],
        config: &'a Config,
        develop: &'a DevelopData,
    ) -> Self {
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
    working_path: PathBuf,
    name: String,
    tera: Tera,
}

impl Template {
    pub fn new(path: impl AsRef<Path>, name: String) -> Result<Self, StapleError> {
        let root = path.as_ref().to_str().unwrap();
        let theme_folder = format!("{}/templates/{}/*", root, name);
        debug!("theme folder is {}", theme_folder);
        let mut tera = Tera::new(&theme_folder)?;
        tera.register_filter("not_field", crate::util::filter::not_field);
        tera.register_filter("markdown", crate::util::filter::markdown);
        tera.register_function("page_detail", crate::util::filter::page_detail);
        Ok(Template {
            working_path: path.as_ref().to_path_buf(),
            name,
            tera,
        })
    }

    pub fn render(
        self,
        articles: Vec<PageInfo>,
        config: &Config,
        is_develop_mode: bool,
    ) -> Result<(), StapleError> {
        Template::remove_folder(self.working_path.join(RENDER_FOLDER))?;
        std::fs::create_dir(self.working_path.join(RENDER_FOLDER))?;

        // todo can be parallel rendering
        for article in articles.iter() {
            self.render_article(config, article, &articles, is_develop_mode)?;
        }

        self.copy_statics_folder(config)?;
        self.copy_statics(config)?;

        Template::remove_folder(self.working_path.join(PUBLIC_FOLDER))?;
        std::fs::rename(
            self.working_path.join(RENDER_FOLDER),
            self.working_path.join(PUBLIC_FOLDER),
        )?;
        Ok(())
    }

    pub fn render_article<'a>(
        &self,
        config: &Config,
        article: &PageInfo,
        articles: &'a [PageInfo],
        is_develop_mode: bool,
    ) -> Result<(), StapleError> {
        info!("rendering article {}({})", &article.title, &article.url);
        let debug_data = DevelopData::new(is_develop_mode);

        let full_article = article.to_full_article()?;

        let data = RenderData::new(full_article, articles, config, &debug_data);
        let context = Context::from_serialize(&data).expect("cannot serialize");
        let result = self.tera.render(data.page.template(), &context)?;
        let url = article.output_file_name();
        let url = &url[1..url.len()];
        let output_file = self.working_path.join(RENDER_FOLDER).join(url);

        if let Some(p) = output_file.parent() {
            if !p.exists() {
                std::fs::create_dir_all(p)?;
            }
        }
        std::fs::write(output_file, result.as_bytes()).map_err(StapleError::IoError)
    }

    fn copy_statics_folder(&self, config: &Config) -> Result<(), StapleError> {
        info!("copy template static folder");
        let statics_folder = self
            .working_path
            .join("templates")
            .join(&config.site.theme)
            .join("statics");
        if statics_folder.exists() {
            debug!("statics folder exist, copy to render folder");
            copy_dir::copy_dir(
                statics_folder,
                self.working_path.join(RENDER_FOLDER).join("statics"),
            )?;
        }
        Ok(())
    }

    pub fn remove_folder(path: impl AsRef<Path>) -> Result<(), StapleError> {
        let buf = path.as_ref();
        if buf.exists() {
            debug!("remove folder {}", buf.to_str().expect("invalid file name"));
            std::fs::remove_dir_all(buf).map_err(StapleError::IoError)
        } else {
            Ok(())
        }
    }

    pub fn copy_statics(&self, config: &Config) -> Result<(), StapleError> {
        let render_folder = self.working_path.join(RENDER_FOLDER);
        for statics in &config.statics {
            let from = self.working_path.join(&statics.from);
            let to = render_folder.join(&statics.to);
            info!("coping statics from {} to {}", &statics.from, &statics.to);
            std::fs::copy(from, to)?;
        }
        Ok(())
    }
}
