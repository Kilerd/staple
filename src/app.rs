use crate::{
    config::Config,
    data::{JsonFileData, MarkdownFileData, PageInfo},
    error::StapleError,
    template::Template,
};
use walkdir::WalkDir;

use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct App {
    pub(crate) config: Config,
    pub(crate) template: Template,
    is_develop_mode: bool,
    path: PathBuf,
}

impl App {
    pub fn load(path: impl AsRef<Path>,  develop: bool) -> Result<Self, StapleError> {
        let config = Config::load_from_file(&path)?;
        debug!("init template");
        let template = Template::new(config.get_theme()?)?;
        Ok(Self {
            config,
            template,
            is_develop_mode: develop,
            path: path.as_ref().to_path_buf()
        })
    }

    pub fn render(self) -> Result<(), StapleError> {
        for x in &self.config.hook.before_build {
            info!("Before-Build Script: {}", x);

            let mut result = std::process::Command::new("sh")
                .arg("-c")
                .arg(x.to_cmd())
                .current_dir(x.to_dir())
                .spawn()
                .expect("cannot spawn process");

            let status = result.wait()?;
            if !status.success() {
                return Err(StapleError::HookError(x.to_cmd(), status.code()));
            }
        }
        let vec = self
            .load_all_data()?
            .into_iter()
            .filter(|article| !article.draw)
            .collect();
        self.template
            .render(vec, &self.config, self.is_develop_mode)?;

        for x in &self.config.hook.after_build {
            info!("Before-Build Script: {}", x);

            let mut result = std::process::Command::new("sh")
                .arg("-c")
                .arg(x.to_cmd())
                .current_dir(x.to_dir())
                .spawn()
                .expect("cannot spawn process");

            let status = result.wait()?;
            if !status.success() {
                return Err(StapleError::HookError(x.to_cmd(), status.code()));
            }
        }

        Ok(())
    }

    pub fn load_all_data(&self) -> Result<Vec<PageInfo>, StapleError> {
        let path = Path::new("data");
        let mut articles = vec![];
        let filter = WalkDir::new(path)
            .into_iter()
            .flat_map(|e| e.ok())
            .filter(|de| de.path().is_file());

        for path in filter {
            let file_path = path.path();
            if file_path.is_file() {
                let extension = file_path.extension().and_then(|e| e.to_str());

                let path = file_path
                    .to_str()
                    .expect("invalid file path encoding")
                    .to_string();
                match extension {
                    Some("md") => {
                        let result2 = MarkdownFileData::load(&path)?;
                        let info = PageInfo {
                            file: path,
                            url: result2.url,
                            title: result2.title,
                            template: result2.template,
                            draw: result2.draw,
                            datetime: result2.datetime,
                            data: result2.data,
                            description: result2.description,
                        };
                        articles.push(info);
                    }
                    Some("json") => {
                        let result = std::fs::read_to_string(file_path)?;
                        let data = JsonFileData::from_str(&result)?;

                        let info = PageInfo {
                            file: path,
                            url: data.url,
                            title: data.title,
                            template: data.template,
                            draw: data.draw,
                            datetime: data.datetime,
                            data: data.data,
                            description: data.description,
                        };

                        articles.push(info);
                    }
                    _ => {}
                }
            }
        }
        articles.sort_by(|one, other| other.datetime.cmp(&one.datetime));
        Ok(articles)
    }
}
