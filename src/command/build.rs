use crate::app::App;
use crate::command::StapleCommand;
use crate::error::StapleError;
use file_lock::FileLock;

pub(crate) fn build() -> Result<(), StapleError> {
    StapleCommand::check_config_file_exist()?;
    let _file_lock = match FileLock::lock("Staple.lock", true, true) {
        Ok(lock) => lock,
        Err(err) => panic!("Error getting write lock: {}", err),
    };

    App::load()?.render()
}
