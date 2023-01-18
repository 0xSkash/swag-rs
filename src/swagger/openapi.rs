use std::fs::File;
use serde_json::Value;
use crate::swagger::info::Info;
use crate::swagger::SwaggerModel;
use crate::util::error::Error;

pub struct OpenAPI {
    pub info: Info,
}

impl OpenAPI {
    pub fn from_path(path: &str) -> Result<OpenAPI, Error> {
        let file_path = File::open(path).map_err(|why| {
            error!("Failed to open Swagger Docs file :: {:?}", why);
            Error::DocsFileNotFound
        })?;

        let json: Value = serde_json::from_reader(&file_path).map_err(|why| {
            error!("Failed to parse JSON from provided file :: {:?}", why);
            Error::JsonParse
        })?;

        Ok(OpenAPI {
            info: Info::from_json(&json).ok_or(Error::InfoParse)?
        })
    }
}