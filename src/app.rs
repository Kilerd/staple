use crate::article::Article;
use crate::config::Config;
use crate::error::StapleError;
use crate::template::Template;

#[derive(Debug)]
pub struct App {
    config: Config,
    template: Template,
}

impl App {
    pub fn load() -> Result<Self, StapleError> {
        let config = Config::load_from_file()?;
        let template = Template::new(config.get_theme()?);
        Ok(Self { config, template })

    }
    pub fn render(self) -> Result<(), StapleError> {
        let articles = Article::load_all_article();
        self.template.render(articles, &self.config)
    }
}
