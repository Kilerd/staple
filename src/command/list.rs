use crate::{app::App, error::StapleError};
use colored::*;
use std::path::Path;

pub(crate) fn command(path: impl AsRef<Path>) -> Result<(), StapleError> {
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
    use crate::{command::list::command, test::setup};

    #[test]
    fn test_list() -> Result<(), Box<dyn std::error::Error>> {
        let dir = setup();
        crate::command::init::init(&dir)?;

        command(dir)?;

        Ok(())
    }
}
