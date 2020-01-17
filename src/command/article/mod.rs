use crate::article::Article;
use crate::error::StapleError;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum ArticleCommand {
    /// create a new article
    New {
        /// specific url of article
        url: String,
        /// tags of article
        #[structopt(long)]
        tags: Vec<String>,
        /// specific title, default is same as url
        #[structopt(short, long)]
        title: Option<String>,
    },
}

impl ArticleCommand {
    pub fn run(&self) -> Result<(), StapleError> {
        match self {
            ArticleCommand::New { url, tags, title } => Article::new_template(url, title, tags),
        }
    }
}
