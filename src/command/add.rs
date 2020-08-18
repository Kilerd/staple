use crate::{
    app::App,
    data::{JsonFileData, MarkdownFileData},
    error::StapleError,
};

pub fn add(
    title: String,
    url: Option<String>,
    template: Option<String>,
    draw: bool,
    data: bool,
) -> Result<(), StapleError> {
    let app = App::load(false)?;
    let url = url.unwrap_or_else(|| title.trim().replace(" ", "-").replace("_", "-"));
    let template = template.unwrap_or(app.config.site.default_template);

    if data {
        // new json file
        JsonFileData::create(title, url, template, draw)
    } else {
        // new markdown file
        MarkdownFileData::create(title, url, template, draw)
    }
}
