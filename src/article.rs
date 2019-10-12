use std::sync::Arc;
use serde_derive::Serialize;

#[derive(Serialize)]
pub struct Article {
    pub url: String
}

impl Article {

    pub fn load_all_article() -> Vec<Article> {

        return vec![
            Article{ url: "1".to_string() },
            Article{ url: "2".to_string() },
            Article{ url: "3".to_string() },
            Article{ url: "4".to_string() },
        ];
    }
}