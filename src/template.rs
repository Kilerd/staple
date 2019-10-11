use tera::{compile_templates, Context, Tera};

use crate::article::Article;
use std::fs::File;
use std::io::{Write, ErrorKind};

pub struct Template {
    name: String,
    tera: Tera,
}

impl Template {
    pub fn new(name: String) -> Self {
        let mut tera = compile_templates!(&format!("templates/{}/**/*", name));
        Template { name, tera }
    }

    pub fn render(self, articles: Vec<Article>) {
        std::fs::remove_dir_all(".render").expect("cannot remove .render folder");
        std::fs::create_dir(".render").expect("cannot create .render folder");
        // index
        let result = self.tera.render("index.html", &Context::new()).expect("cannot found index.html");
        let mut result1 = File::create(".render/index.html").expect("cannot open render file");
        result1.write_all(result.as_bytes());

        // article
        std::fs::create_dir(".render/articles").expect("cannot create .render/articles folder");

        articles.into_iter().for_each(|article| {
            let mut context = Context::new();
            context.insert("article", &article);
            let result = self.tera.render("article.html", &context).expect("cannot found article.html");
            let mut result1 = File::create(format!(".render/articles/{}.html", article.url)).expect("cannot open render file");
            result1.write_all(result.as_bytes());
        });

        let result2 = std::fs::remove_dir_all("public");
        if let Err(e) = result2 {
            if e.kind() != ErrorKind::NotFound {
                panic!(e);
            }
        }
        std::fs::rename(".render", "public");
    }
}
