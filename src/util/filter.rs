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
            field.is_null()
                || field.eq(&Value::Bool(false))
                || field.eq(&Value::String(String::from("false")))
                || field.eq(&Value::String(String::from("False")))
                || field.eq(&Value::String(String::from("FALSE")))
        })
        .collect();

    Ok(result)
}
