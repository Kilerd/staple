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
        info!(
            "{} folder {} existed, please delete it then run `new` again, or just use `--force` flag (it would delete existed folder and create a new one)",
            style("ERROR").red(),
            style(path).blue()
        );
        return Ok(());
    }

    std::fs::create_dir(buf)?;
    crate::command::init::init(path.as_str())
}
