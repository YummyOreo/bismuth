use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "bismuth")]
#[command(about = "A static site gen", long_about = None)]
pub struct CliArguments {
    #[command(subcommand)]
    pub command: Commands,
}
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Builds the app in the current directory
    Build,
    /// Inits a new project
    Init {
        /// Name of the project to create
        name: String
    }
}

pub fn parse_args() -> CliArguments {
    CliArguments::parse()
}
