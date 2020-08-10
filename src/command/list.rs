use crate::{app::App, error::StapleError};

pub(crate) fn command() -> Result<(), StapleError> {
    let result = App::load(false)?;

    info!("Project Name: {}", result.config.site.title);

    Ok(())
}
