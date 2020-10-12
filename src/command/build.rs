use crate::{app::App, command::StapleCommand, error::StapleError};
use std::path::Path;

pub(crate) fn build(path: impl AsRef<Path>, develop: bool) -> Result<(), StapleError> {
    StapleCommand::lock_file()?;
    App::load(&path, develop)?.render()
}
