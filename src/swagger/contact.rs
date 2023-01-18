use serde_json::Value;
use crate::swagger::{get_string_for_key, SwaggerModel};

pub struct Contact {
    pub name: Option<String>,
    pub url: Option<String>,
    pub email: Option<String>,
}

impl SwaggerModel<Contact> for Contact {
    fn from_json(json: &Value) -> Option<Contact> {
        let root = json.get(Contact::key())?;

        let name = get_string_for_key("name", root);
        let url = get_string_for_key("url", root);
        let email = get_string_for_key("email", root);

        Some(Contact {
            name,
            url,
            email,
        })
    }

    fn key() -> String {
        "contact".to_owned()
    }
}