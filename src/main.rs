mod swagger;
mod util;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use pretty_env_logger::env_logger;
use crate::swagger::openapi::OpenAPI;

fn main() {
    std::env::set_var("RUST_LOG", "trace");
    env_logger::init();

    let open_api = OpenAPI::from_path("./example-data/openapi.json");

    match open_api {
        Ok(api) => {
            info!("Open API parsed with name: {}, version: {}", &api.info.title, &api.info.version)
        }
        Err(why) => {
            info!("Error: {}", why)
        }
    }
}
