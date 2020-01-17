use crate::{article::Article, config::Config, error::StapleError, template::Template};

#[derive(Debug)]
pub struct App {
    pub(crate) config: Config,
    pub(crate) template: Template,
}

impl App {
    pub fn load() -> Result<Self, StapleError> {
        let config = Config::load_from_file()?;
        debug!("init template");
        let template = Template::new(config.get_theme()?);
        Ok(Self { config, template })
    }
    pub fn render(self) -> Result<(), StapleError> {
        debug!("starting render");
        debug!("load all article");
        let articles = Article::load_all_article();
        self.template.render(articles?, &self.config)
    }
}
