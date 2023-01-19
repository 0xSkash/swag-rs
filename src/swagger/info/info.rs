use serde_json::Value;
use crate::swagger::{get_string_for_key, SwaggerModel};
use crate::swagger::info::contact::Contact;
use crate::swagger::info::licence::Licence;

pub struct Info {
    pub title: String,
    pub description: Option<String>,
    pub terms_of_service: Option<String>,
    pub contact: Option<Contact>,
    pub licence: Option<Licence>,
    pub version: String,
}

impl SwaggerModel<Info> for Info {
    fn from_json(json: &Value) -> Option<Self> {
        let root = json.get(Info::key())?;

        let title = get_string_for_key("title", root)?;
        let description = get_string_for_key("description", root);
        let contact = Contact::from_json(root);
        let licence = Licence::from_json(root);
        let terms_of_service = get_string_for_key("termsOfService", root);
        let version = get_string_for_key("version", root)?;

        Some(Info {
            title,
            description,
            terms_of_service,
            contact,
            licence,
            version,
        })
    }

    fn key() -> String {
        "info".to_owned()
    }
}