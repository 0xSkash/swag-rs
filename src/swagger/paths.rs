use std::collections::HashMap;
use serde_json::Value;
use crate::swagger::path::Path;
use crate::swagger::SwaggerModel;

pub struct Paths {
    pub paths: HashMap<String, Path>,
}

impl SwaggerModel<Paths> for Paths {
    fn from_json(json: &Value) -> Option<Paths> {
        todo!()
    }

    fn key() -> String {
        "paths".to_owned()
    }
}