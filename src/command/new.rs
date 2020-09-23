use crate::{error::StapleError, template::Template};
use console::style;
use std::path::Path;

pub(crate) fn new(path: String, _title: Option<String>, force: bool) -> Result<(), StapleError> {
    let buf = Path::new(".").join(&path);
    if force {
        // TODO add print
        Template::remove_folder(path.as_str())?;
    }
    if buf.as_path().exists() {
        error!(
            "folder {} existed, please delete it then run `new` again, or just use `--force` flag (it would delete existed folder and create a new one)",
            style(path).blue()
        );
        return Ok(());
    }

    std::fs::create_dir(buf)?;
    crate::command::init::init(path.as_str())
}

#[cfg(test)]
mod test {
    use crate::{command::new::new, constants::STAPLE_CONFIG_FILE};

    #[test]
    fn should_show_error_when_folder_exists() -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempfile::tempdir()?.into_path();
        std::env::set_current_dir(&dir)?;

        let existed_folder = dir.join("exists");
        let existed_file = existed_folder.join(STAPLE_CONFIG_FILE);
        std::fs::create_dir(&existed_folder)?;
        std::fs::write(&existed_file, "invalid config")?;
        new("exists".to_owned(), None, false)?;

        let string = std::fs::read_to_string(&existed_file)?;
        assert_eq!("invalid config", string);
        Ok(())
    }

    #[test]
    fn should_delete_existed_folder_when_force_flag_is_true(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempfile::tempdir()?.into_path();
        std::env::set_current_dir(&dir)?;

        let existed_folder = dir.join("exists");
        let existed_file = existed_folder.join(STAPLE_CONFIG_FILE);
        std::fs::create_dir(&existed_folder)?;
        std::fs::write(&existed_file, "invalid config")?;
        new("exists".to_owned(), None, true)?;

        let string = std::fs::read_to_string(&existed_file)?;
        assert_ne!("invalid config", string);
        Ok(())
    }
}
