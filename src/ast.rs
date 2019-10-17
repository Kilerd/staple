use std::collections::HashMap;

#[derive(Debug)]
pub struct Article {
    pub meta: Vec<(String, String)>,
    pub content: String,
}
