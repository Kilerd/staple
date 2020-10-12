use crate::{app::App, command::StapleCommand, error::StapleError};
use std::path::Path;

pub(crate) fn build(path: impl AsRef<Path>, develop: bool) -> Result<(), StapleError> {
    StapleCommand::lock_file(&path)?;
    App::load(&path, develop)?.render()
}

#[cfg(test)]
mod test {
    use crate::{
        command::{
            add::{add, AddOptions},
            build::build,
        },
        test::setup,
    };

    #[test]
    fn should_render_article_content() -> Result<(), Box<dyn std::error::Error>> {
        let dir = setup();
        crate::command::init::init(&dir)?;
        let options = AddOptions {
            title: "test-markdown".to_owned(),
            url: None,
            template: None,
            draw: false,
            data: false,
        };
        add(&dir, options)?;

        let article = dir.join("data/test-markdown.md");
        let string = std::fs::read_to_string(&article)?;
        let string1 = format!("{}\n\n{}", string, "# hello");
        std::fs::write(&article, string1)?;
        build(&dir, false)?;

        let x = "<h1>hello</h1>\n";
        assert_eq!(
            x,
            std::fs::read_to_string(dir.join("public/test-markdown/index.html"))?
        );

        Ok(())
    }

    // todo should_render_json_content
    // todo metadata_should_be_rendered

    // todo cross access article data
    // todo develop mode rendering
}
