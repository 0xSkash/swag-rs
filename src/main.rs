mod swagger;
mod util;
mod cli;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use std::path::Path;
use clap::Parser;
use pretty_env_logger::env_logger;
use crate::cli::args::{CLIArgs, Command, GeneratorArgs, GeneratorSubcommand, GeneratorType};
use crate::swagger::openapi::OpenAPI;
use crate::util::error::Error;

fn main() {
    std::env::set_var("RUST_LOG", "trace");
    env_logger::init();

    let args = CLIArgs::parse();

    match &args.command {
        Command::Generate(sub_command) => {
            if let Err(why) = handle_generate_command(sub_command) {
                info!("generate command failed with error {:?}", why)
            }
        }
    }
}

fn handle_generate_command(command: &GeneratorSubcommand) -> Result<(), Error> {
    let args = get_args_from_cli_input(command);
    validate_args(args)?;

    let openapi = OpenAPI::from_path(&args.input_path)?;

    info!("Found OpenAPI with title: {}, version: {}", &openapi.info.title, &openapi.info.version);

    let _ = &openapi.components.schemas.values.iter().for_each(|(key, schema)| {
        info!("Found Schema {:?} with type {:?}", key, schema.schema_type)
    });

    // TODO: Implement API-Client generation
    Ok(())
}

fn get_args_from_cli_input(sub_command: &GeneratorSubcommand) -> &GeneratorArgs {
    match &sub_command.command {
        GeneratorType::Android(args) => args,
        GeneratorType::Ios(args) => args
    }
}

fn validate_args(args: &GeneratorArgs) -> Result<(), Error> {
    let input_path = Path::new(&args.input_path);
    let output_path = Path::new(&args.output_path);

    if !input_path.exists() {
        error!("Provided path {:?} does not exist", input_path);
        return Err(Error::InvalidPath);
    }

    if !output_path.exists() {
        error!("Provided path {:?} does not exist", output_path);
        return Err(Error::InvalidPath);
    }

    Ok(())
}
