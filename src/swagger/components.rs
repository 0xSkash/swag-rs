use std::collections::HashMap;
use serde_json::Value;
use crate::swagger::callback::Callback;
use crate::swagger::example::Example;
use crate::swagger::header::Header;
use crate::swagger::link::Link;
use crate::swagger::parameter::Parameter;
use crate::swagger::request_body::RequestBody;
use crate::swagger::response::Response;
use crate::swagger::schema::Schema;
use crate::swagger::schemas::Schemas;
use crate::swagger::security_schema::SecuritySchema;
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