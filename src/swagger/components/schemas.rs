use std::collections::HashMap;
use serde_json::Value;
use crate::swagger::components::schema::Schema;
use crate::swagger::SwaggerModel;

pub struct Schemas {
    pub values: HashMap<String, Schema>,
}

impl SwaggerModel<Schemas> for Schemas {
    fn from_json(json: &Value) -> Option<Schemas> {
        let root = json.get(Schemas::key())?.as_object()?;

        let mut values: HashMap<String, Schema> = HashMap::new();

        for key in root.keys() {
            let schema = Schema::from_json(root.get(key)?)?;
            values.insert(key.to_owned(), schema);
        }

        Some(Schemas {
            values
        })
    }

    fn key() -> String {
        "schemas".to_owned()
    }
}

