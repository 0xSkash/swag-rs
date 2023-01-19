use serde_json::Value;
use crate::swagger::{get_string_for_key, SwaggerModel};

pub struct ExternalDocs {
    pub description: Option<String>,
    pub url: String,
}

impl SwaggerModel<ExternalDocs> for ExternalDocs {
    fn from_json(json: &Value) -> Option<ExternalDocs> {
        let root = json.get(ExternalDocs::key())?;

        let description = get_string_for_key("description", root);
        let url = get_string_for_key("url", root)?;

        Some(ExternalDocs {
            description,
            url,
        })
    }

    fn key() -> String {
        "externalDocs".to_owned()
    }
}