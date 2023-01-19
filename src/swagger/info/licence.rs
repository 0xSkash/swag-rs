use serde_json::Value;
use crate::swagger::{get_string_for_key, SwaggerModel};

pub struct Licence {
    pub name: Option<String>,
    pub url: Option<String>,
}

impl SwaggerModel<Licence> for Licence {
    fn from_json(json: &Value) -> Option<Licence> {
        let root = json.get(Licence::key())?;

        let name = get_string_for_key("name", root);
        let url = get_string_for_key("url", root);

        Some(Licence {
            name,
            url,
        })
    }

    fn key() -> String {
        "licence".to_owned()
    }
}