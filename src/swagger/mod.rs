use serde_json::Value;

pub mod openapi;
mod info;
mod external_docs;
mod components;
mod security_schema;
mod paths;

trait SwaggerModel<T> {
    fn from_json(json: &Value) -> Option<T>;
    fn key() -> String;
}

fn get_string_for_key(key: &str, json: &Value) -> Option<String> {
    let value = json.get(key).and_then(|value| value.as_str());

    if let Some(string) = value {
        return Some(string.to_owned());
    }
    None
}

