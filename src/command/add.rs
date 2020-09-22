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
