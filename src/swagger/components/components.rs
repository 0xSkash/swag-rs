use std::collections::HashMap;
use serde_json::Value;
use crate::swagger::components::schemas::Schemas;
use crate::swagger::SwaggerModel;

pub struct Components {
    pub schemas: Schemas,
//    pub responses: HashMap<String, Response>,
//    pub parameters: HashMap<String, Parameter>,
//    pub examples: HashMap<String, Example>,
//    pub request_bodies: HashMap<String, RequestBody>,
//    pub headers: HashMap<String, Header>,
//    pub security_schemas: HashMap<String, SecuritySchema>,
//    pub callbacks: HashMap<String, Callback>,
}

impl SwaggerModel<Components> for Components {
    fn from_json(json: &Value) -> Option<Components> {
        let root = json.get(Components::key())?;

        let schemas = Schemas::from_json(root)?;

        Some(Components {
            schemas
        })
    }

    fn key() -> String {
        "components".to_owned()
    }
}
