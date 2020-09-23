use crate::{
    app::App,
    data::types::{json::JsonFileData, markdown::MarkdownFileData, CreationOptions, FileType},
    error::StapleError,
};
use std::path::Path;

pub fn add(
    title: String,
    url: Option<String>,
    template: Option<String>,
    draw: bool,
    data: bool,
) -> Result<(), StapleError> {
    let path = Path::new("./");
    let app = App::load(&path, false)?;
    let url = url.unwrap_or_else(|| title.trim().replace(" ", "-").replace("_", "-"));
    let template = template.unwrap_or(app.config.site.default_template);

    // new json file
    let options = CreationOptions {
        title,
        url,
        template,
        draw,
    };
    if data {
        JsonFileData::create(path, &options)
    } else {
        MarkdownFileData::create(path, &options)
    }
}

#[cfg(test)]
mod test {
    use crate::command::add::add;
    use serde_json::Value;
    #[test]
    fn should_add_markdown_file() -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempfile::tempdir()?.into_path();
        std::env::set_current_dir(&dir)?;
        crate::command::init::init("./")?;

        add("test-one".to_owned(), None, None, false, false)?;
        assert!(dir.join("data").join("test-one.md").exists());

        Ok(())
    }
    #[test]
    fn should_add_json_file() -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempfile::tempdir()?.into_path();
        std::env::set_current_dir(&dir)?;
        crate::command::init::init("./")?;

        add("test-one".to_owned(), None, None, false, true)?;
        assert!(dir.join("data").join("test-one.json").exists());

        Ok(())
    }

    #[test]
    fn should_add_json_draw_file() -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempfile::tempdir()?.into_path();
        std::env::set_current_dir(&dir)?;
        crate::command::init::init("./")?;

        add("test-one".to_owned(), None, None, true, true)?;
        let buf = dir.join("data").join("test-one.json");
        let string = std::fs::read_to_string(buf)?;
        let result: serde_json::Value = serde_json::from_str(&string)?;

        assert_eq!(Some(&Value::Bool(true)), result.get("draw"));
        Ok(())
    }
}
