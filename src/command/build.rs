use crate::{app::App, command::StapleCommand, error::StapleError};
use std::path::Path;

pub(crate) fn build(develop: bool) -> Result<(), StapleError> {
    StapleCommand::lock_file()?;
    let path = Path::new("./");
    App::load(&path, develop)?.render()
}
