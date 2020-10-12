use crate::{app::App, command::StapleCommand, error::StapleError};
use colored::*;
use std::path::Path;

pub(crate) fn command(path: impl AsRef<Path>) -> Result<(), StapleError> {
    StapleCommand::lock_file(&path)?;
    let app = App::load(path, false)?;
    info!("Project Name: {}", app.config.site.title);
    let mut pages = app.load_all_data()?;
    pages.reverse();
    for page in pages {
        let draw = if page.draw { "DRAW" } else { "" };

        info!(
            "{} {:4} {}({})",
            page.datetime.format("%b %d, %Y"),
            draw.blue(),
            page.title.white().magenta(),
            page.url
        );
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{
        command::{
            add::{add, AddOptions},
            list::command,
        },
        test::setup,
    };

    #[test]
    fn test_list() -> Result<(), Box<dyn std::error::Error>> {
        let dir = setup();
        crate::command::init::init(&dir)?;

        command(dir)?;

        Ok(())
    }

    #[test]
    fn should_show_article_list() -> Result<(), Box<dyn std::error::Error>> {
        let dir = setup();
        crate::command::init::init(&dir)?;
        let options = AddOptions {
            title: "test-one".to_owned(),
            url: None,
            template: None,
            draw: true,
            data: true,
        };
        add(&dir, options)?;

        let options = AddOptions {
            title: "test-two".to_owned(),
            url: None,
            template: None,
            draw: false,
            data: false,
        };
        add(&dir, options)?;

        Ok(command(dir)?)
    }
}
