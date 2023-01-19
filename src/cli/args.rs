use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct CLIArgs {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Generate API-Clients from OpenAPI-Docs
    Generate(GeneratorSubcommand)
}

#[derive(Debug, Args)]
pub struct GeneratorSubcommand {
    #[clap(subcommand)]
    pub command: GeneratorType,
}

#[derive(Debug, Subcommand)]
pub enum GeneratorType {
    /// Generates API-Client for Android
    Android(GeneratorArgs),
    /// Generates API-Client for iOS
    Ios(GeneratorArgs),
}

#[derive(Debug, Args)]
pub struct GeneratorArgs {
    #[arg(short, long)]
    /// Path pointing to the OpenAPI-Docs Json File
    pub input_path: String,
    #[arg(short, long)]
    /// Path pointing to the directory where you want swag to store the generated API-Client
    pub output_path: String,
}