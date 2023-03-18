use bismuth_html::{render_list, Renderer};
use bismuth_lexer::Lexer;
use bismuth_md::MarkdownFile;
use bismuth_parser::Parser;
use std::path::{Path, PathBuf};

mod arguments;
pub mod config;
mod init;

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

pub fn render(files: Vec<Parser>) -> Vec<(Renderer, String)> {
    render_list(files)
}

pub fn write(files: Vec<(Renderer, String)>) -> Result<(), std::io::Error> {
    for file in files {
        file.0.write()?;
    }
    Ok(())
}

pub fn run(dir: String) {
    let path = Path::new(&dir).canonicalize().unwrap();

    let _config = config::Config::new(&path);

    let mut src_path = path.clone();
    src_path.push("src/");
    let md_files = get_files(&src_path);
    let tokenized_file = run_lexer(md_files);
    let parsers = run_parser(tokenized_file);
    let renderers = render(parsers);
    write(renderers).unwrap();
}

pub fn entry(dir: String) {
    let args = arguments::parse_args();

    match args.command {
        arguments::Commands::Run => run(dir),
        arguments::Commands::Init { name } => {
            init::init_folder(&name).unwrap();
        }
    }
}
