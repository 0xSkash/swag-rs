use serde_json::Value;
use crate::swagger::{get_string_for_key, SwaggerModel};
use crate::swagger::components::properties::Properties;

pub struct Schema {
    pub schema_type: String,
}

impl SwaggerModel<Schema> for Schema {
    fn from_json(json: &Value) -> Option<Schema> {
        let schema_type = get_string_for_key("type", json)?;

        Some(Schema {
            schema_type
        })
    }

    fn key() -> String {
        // Not needed
        "".to_owned()
    }
}


