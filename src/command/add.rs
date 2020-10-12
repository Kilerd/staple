use crate::{
    app::App,
    data::types::{json::JsonFileData, markdown::MarkdownFileData, CreationOptions, FileType},
    error::StapleError,
};
use std::path::Path;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct AddOptions {
    pub title: String,
    #[structopt(long)]
    pub url: Option<String>,
    #[structopt(long, short)]
    pub template: Option<String>,
    #[structopt(long)]
    pub draw: bool,
    #[structopt(long)]
    pub data: bool,
}

pub fn add(path: impl AsRef<Path>, options: AddOptions) -> Result<(), StapleError> {
    let app = App::load(&path, false)?;
    let url = options.url.clone().unwrap_or_else(|| {
        options
            .title
            .clone()
            .trim()
            .replace(" ", "-")
            .replace("_", "-")
    });
    let template = options.template.unwrap_or(app.config.site.default_template);

    // new json file
    let create_options = CreationOptions {
        title: options.title,
        url,
        template,
        draw: options.draw,
    };
    if options.data {
        JsonFileData::create(path, &create_options)
    } else {
        MarkdownFileData::create(path, &create_options)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        command::add::{add, AddOptions},
        test::setup,
    };
    use serde_json::Value;

    #[test]
    fn should_add_markdown_file() -> Result<(), Box<dyn std::error::Error>> {
        let dir = setup();

        crate::command::init::init(&dir)?;

        let options = AddOptions {
            title: "test-one".to_owned(),
            url: None,
            template: None,
            draw: false,
            data: false,
        };
        add(&dir, options)?;
        assert!(dir.join("data").join("test-one.md").exists());

        Ok(())
    }
    #[test]
    fn should_add_json_file() -> Result<(), Box<dyn std::error::Error>> {
        let dir = setup();
        crate::command::init::init(&dir)?;
        let options = AddOptions {
            title: "test-one".to_owned(),
            url: None,
            template: None,
            draw: false,
            data: true,
        };
        add(&dir, options)?;
        assert!(dir.join("data").join("test-one.json").exists());

        Ok(())
    }

    #[test]
    fn should_add_json_draw_file() -> Result<(), Box<dyn std::error::Error>> {
        let dir = setup();
        crate::command::init::init(&dir)?;
        debug!("staple working in {}", dir.to_str().unwrap());
        let options = AddOptions {
            title: "test-one".to_owned(),
            url: None,
            template: None,
            draw: true,
            data: true,
        };
        add(&dir, options)?;
        let buf = dir.join("data").join("test-one.json");
        let string = std::fs::read_to_string(buf)?;
        let result: serde_json::Value = serde_json::from_str(&string)?;

        assert_eq!(Some(&Value::Bool(true)), result.get("draw"));
        Ok(())
    }
}
