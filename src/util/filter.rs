use crate::data::PageInfo;
use chrono::{FixedOffset, Utc};
use std::collections::HashMap;
use tera::{Error, Value};

pub fn get_json_pointer(key: &str) -> String {
    ["/", &key.replace(".", "/")].join("")
}

/// item of array contains false bool field or do not contains field.
pub fn not_field(value: &Value, attributes: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    let arr = tera::try_get_value!("filter", "value", Vec<Value>, value);

    let key = match attributes.get("attribute") {
        Some(val) => tera::try_get_value!("filter", "attribute", String, val),
        None => {
            return Err(Error::msg(
                "The `not_field` filter has to have an `attribute` argument",
            ))
        }
    };
    let result = arr
        .into_iter()
        .filter(|item| {
            let field = item
                .pointer(&get_json_pointer(&key))
                .unwrap_or(&Value::Null);
            field.is_null()
                || field.eq(&Value::Bool(false))
                || field.eq(&Value::String(String::from("false")))
                || field.eq(&Value::String(String::from("False")))
                || field.eq(&Value::String(String::from("FALSE")))
        })
        .collect();

    Ok(result)
}

/// loading page detail of specific article while rendering.
/// using this to add avalibility and flexibility to render cross-articles page like rss page or those need at least 2 articles full content.
pub fn page_detail(args: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    let file = match args.get("file") {
        Some(val) => match tera::from_value::<String>(val.clone()) {
            Ok(parsed_val) => parsed_val,
            Err(_) => {
                return Err(Error::msg(format!(
                    "Function `page_detail` receive file={} but `file` can only be a string",
                    val
                )))
            }
        },
        None => {
            return Err(Error::msg(
                "Function `page_detail` was called without argument `file`",
            ))
        }
    };

    info!("├── cross accessing page detail {}", &file);
    let full_article = PageInfo {
        file,
        url: "".to_string(),
        title: "".to_string(),
        template: "".to_string(),
        draw: false,
        datetime: Utc::now().with_timezone(&FixedOffset::east(60 * 60 * 8)),
        data: HashMap::new(),
        description: None,
    }
    .to_full_article();
    let data = match full_article {
        Ok(data) => data,
        Err(e) => return Err(Error::msg(format!("Error on loading page detail: {}", e))),
    };
    serde_json::to_value(data).map_err(|_| Error::msg("Error on serializing page data into json"))
}
