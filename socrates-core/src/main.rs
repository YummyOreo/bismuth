use std::path::Path;

mod arguments;
mod config;

// TODO: change this to remove --dir and just take a dir. Also, make it so there has to be a
// socrates.toml file
fn get_files(path: &Path) -> Vec<socrates_md::MarkdownFile> {
    match socrates_md::load::load_from_dir(&path.to_path_buf()) {
        Ok(files) => files,
        Err(e) => panic!("{e:#?}"),
    }
}

fn main() {
    let args = arguments::parse_args();

    match args.command {
        arguments::Commands::Run => {
            let path = Path::new("./").canonicalize().unwrap();

            let _config = config::Config::new(&path);
            let md_files = get_files(&path);
            let mut lexer_files: Vec<socrates_lexer::Lexer> = vec![];
            for file in md_files {
                let mut lexer = socrates_lexer::Lexer::new(file);
                lexer.run_lexer().unwrap();
                lexer_files.push(lexer);
            }

            println!("{lexer_files:#?}");
        }
    }
}
