use crate::article::Article;
use crate::config::Config;
use crate::error::StapleError;
use crate::template::Template;

#[derive(Debug)]
pub struct App {
    config: Config,
}

impl App {
    pub fn load() -> Result<Self, StapleError> {
        let config = Config::load_from_file()?;
        Ok(Self { config })
    }
    pub fn render(self) -> Result<(), StapleError> {
        let articles = Article::load_all_article();
        let template = Template::new("rubble".to_string());
        template.render(articles)
    }
}
