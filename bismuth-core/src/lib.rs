mod arguments;
mod build;
pub mod config;
mod init;

pub fn entry(dir: String) {
    let args = arguments::parse_args();

    match args.command {
        arguments::Commands::Build => build::build(dir),
        arguments::Commands::Init { name } => {
            init::init_folder(&name).unwrap();
        }
    }
}
