use serde_json::Value;

pub mod openapi;
mod contact;
mod info;
mod licence;
mod external_docs;
mod components;
mod schema;
mod response;
mod parameter;
mod example;
mod request_body;
mod header;
mod security_schema;
mod link;
mod callback;
mod paths;
mod path;
mod properties;
mod schemas;

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

