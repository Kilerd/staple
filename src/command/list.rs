use crate::{app::App, error::StapleError};
use colored::*;
use std::path::Path;

pub(crate) fn command() -> Result<(), StapleError> {
    let path = Path::new("./");
    let app = App::load(&path, false)?;
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
