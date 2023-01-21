use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(name = "socrates")]
#[command(about = "A static site gen", long_about = None)]
pub struct CliArguments {
    #[command(subcommand)]
    pub command: Commands,
}
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Run the app in the current directory
    Run,
}

pub fn parse_args() -> CliArguments {
    CliArguments::parse()
}
