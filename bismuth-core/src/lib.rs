use bismuth_lexer::Lexer;
use bismuth_md::MarkdownFile;
use bismuth_parser::Parser;
use std::path::{Path, PathBuf};

mod arguments;
pub mod config;
// TODO: change this to remove --dir and just take a dir. Also, make it so there has to be a
// bismuth.toml file
pub fn get_files(path: &PathBuf) -> Vec<MarkdownFile> {
    bismuth_md::load::load_from_dir(&path).unwrap()
}

pub fn run_lexer(files: Vec<MarkdownFile>) -> Vec<Lexer> {
    files
        .iter()
        .map(|file| {
            let mut lexer = Lexer::new(file.clone());
            lexer.run_lexer().unwrap();
            lexer
        })
        .collect::<Vec<Lexer>>()
}

pub fn run_parser(files: Vec<Lexer>) -> Vec<Parser> {
    let mut parsed_files = files
        .iter()
        .map(|lexer| {
            let mut parser = Parser::new(lexer.clone());
            parser.parse().unwrap();
            Some(parser)
        })
        .collect::<Vec<Option<Parser>>>();

    let mut index = 0;
    while index < parsed_files.len() {
        let file = parsed_files[index].take().expect("Should be there");
        let parsered = bismuth_custom::parse_custom(
            file,
            &parsed_files
                .iter()
                .map(|e| e.as_ref())
                .collect::<Vec<Option<&Parser>>>(),
        );
        parsed_files[index] = Some(parsered);
        index += 1;
    }
    parsed_files
        .iter()
        .map(|e| e.clone().expect("Should be there"))
        .collect::<Vec<Parser>>()
}

pub fn run(dir: String) {
    let path = Path::new(&dir).canonicalize().unwrap();

    let _config = config::Config::new(&path);

    let md_files = get_files(&path);
    let tokenized_file = run_lexer(md_files);
    println!("{:#?}", run_parser(tokenized_file));
}

pub fn entry(dir: String) {
    let args = arguments::parse_args();

    match args.command {
        arguments::Commands::Run => run(dir),
    }
}
