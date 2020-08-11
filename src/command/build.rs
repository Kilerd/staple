use crate::{app::App, command::StapleCommand, error::StapleError};

pub(crate) fn build(develop: bool) -> Result<(), StapleError> {
    StapleCommand::lock_file()?;
    App::load(develop)?.render()
}
