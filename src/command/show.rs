use crate::app::App;
use crate::article::{Article, ArticleMeta};
use crate::error::StapleError;
pub(crate) fn show() -> Result<(), StapleError> {
    let result = App::load()?;
    let article_meta: Vec<Article> = Article::load_all_article()?
        .into_iter()
        .collect();

    println!("Project Name: {}", result.config.site.title);
    println!("article count: {}", article_meta.len());

    Ok(())
}
