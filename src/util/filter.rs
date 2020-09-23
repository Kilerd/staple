use crate::data::{MarkdownContent, PageInfo};
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
            ));
        }
    };
    let result = arr
        .into_iter()
        .filter(|item| {
            let field = item
                .pointer(&get_json_pointer(&key))
                .unwrap_or(&Value::Null);
            if let Value::String(content) = field {
                content.to_uppercase().eq("FALSE")
            } else {
                field.is_null() || field.eq(&Value::Bool(false))
            }
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
                )));
            }
        },
        None => {
            return Err(Error::msg(
                "Function `page_detail` was called without argument `file`",
            ));
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

/// render text as markdown
pub fn markdown(value: &Value, _attributes: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    if let Value::String(content) = value {
        let html_content = MarkdownContent::new(content.to_owned()).html;
        Ok(Value::String(html_content))
    } else {
        Ok(value.to_owned())
    }
}

#[cfg(test)]
mod test {
    use crate::util::filter::{get_json_pointer, markdown, not_field};
    use serde_json::{Map, Value};
    use std::collections::HashMap;
    use tera::ErrorKind;

    #[test]
    fn test_get_json_pointer() {
        assert_eq!("/a", get_json_pointer("a"));
        assert_eq!("/a/b", get_json_pointer("a.b"));
    }

    #[test]
    fn should_raise_error_when_attribute_is_empty_in_not_field() {
        let mut map = Map::new();
        map.insert("ok".to_owned(), Value::Bool(true));
        let value = Value::Array(vec![Value::Object(map)]);
        let result = not_field(&value, &HashMap::new());
        assert!(result.is_err());
    }
    #[test]
    fn test_not_field1() {
        let mut map = Map::new();
        map.insert("ok".to_owned(), Value::Bool(true));
        let value = Value::Array(vec![Value::Object(map)]);

        let mut attribute = HashMap::new();
        attribute.insert("attribute".to_owned(), Value::String("ok".to_owned()));
        let result = not_field(&value, &attribute).unwrap();
        assert_eq!(Value::Array(vec![]), result);
    }
    #[test]
    fn test_not_field2() {
        let mut map1 = Map::new();
        map1.insert("ok".to_owned(), Value::Bool(false));

        let mut map2 = Map::new();
        map2.insert("ok".to_owned(), Value::Null);

        let mut map3 = Map::new();
        map3.insert("ok".to_owned(), Value::String("false".to_owned()));

        let mut map4 = Map::new();
        map4.insert("ok".to_owned(), Value::String("False".to_owned()));

        let mut map5 = Map::new();
        map5.insert("ok".to_owned(), Value::String("FALSE".to_owned()));

        let value = Value::Array(vec![
            Value::Object(map1),
            Value::Object(map2),
            Value::Object(map3),
            Value::Object(map4),
            Value::Object(map5),
        ]);

        let mut attribute = HashMap::new();
        attribute.insert("attribute".to_owned(), Value::String("ok".to_owned()));
        let result = not_field(&value, &attribute).unwrap();
        assert_eq!(5, result.as_array().unwrap().len());
    }

    #[test]
    fn should_render_text_into_markdown() {
        let value = Value::String("hello".to_owned());
        let result = markdown(&value, &HashMap::new()).expect("is not a ok");
        assert_eq!(Value::String("<p>hello</p>\n".to_owned()), result);
    }

    #[test]
    fn should_return_the_same_if_value_is_not_text() {
        let value = Value::Bool(true);
        let result = markdown(&value, &HashMap::new()).expect("is not a ok");
        assert_eq!(Value::Bool(true), result);
    }
}
