use crate::{error::StapleError, template::Template};
use colored::*;
use console::style;
use std::path::Path;

pub(crate) fn new(
    path: impl AsRef<Path>,
    _title: Option<String>,
    force: bool,
) -> Result<(), StapleError> {
    let buf = path.as_ref();
    let folder = buf.to_str().expect("invalid filename");

    if force {
        // TODO add print
        info!(
            "Passing `--force` flag, deleting existed folder {}",
            folder.blue()
        );
        Template::remove_folder(&path)?;
    }
    if buf.exists() {
        let filename = buf.to_str().expect("invalid file name");
        error!(
            "folder {} existed, please delete it then run `new` again, or just use `--force` flag (it would delete existed folder and create a new one)",
            style(filename).blue()
        );
        return Ok(());
    }
    info!("Creating staple folder {}", folder.blue());
    std::fs::create_dir(buf)?;
    crate::command::init::init(&path)
}

#[cfg(test)]
mod test {
    use crate::{command::new::new, constants::STAPLE_CONFIG_FILE, test::setup};

    #[test]
    fn should_show_error_when_folder_exists() -> Result<(), Box<dyn std::error::Error>> {
        let dir = setup();

        let existed_folder = dir.join("exists");
        let existed_file = existed_folder.join(STAPLE_CONFIG_FILE);
        std::fs::create_dir(&existed_folder)?;
        std::fs::write(&existed_file, "invalid config")?;
        new(existed_folder, None, false)?;

        let string = std::fs::read_to_string(&existed_file)?;
        assert_eq!("invalid config", string);
        Ok(())
    }

    #[test]
    fn should_delete_existed_folder_when_force_flag_is_true(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let dir = setup();

        let existed_folder = dir.join("exists");
        std::fs::create_dir(&existed_folder)?;

        let existed_file = existed_folder.join(STAPLE_CONFIG_FILE);
        std::fs::write(&existed_file, "invalid config")?;

        new(existed_folder, None, true)?;

        let string = std::fs::read_to_string(&existed_file)?;
        assert_ne!("invalid config", string);
        Ok(())
    }
}
