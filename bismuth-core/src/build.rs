use bismuth_html::render_list;
use bismuth_lexer::Lexer;
use bismuth_md::MarkdownFile;
use bismuth_parser::Parser;
use bismuth_tui::prompt::{builtin::YesNo, Input};
use std::path::{Path, PathBuf};

use crate::config::Config;

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

pub fn build(dir: String) {
    let path = Path::new(&dir).canonicalize().unwrap();

    let _config = Config::new(&path);

    let mut src_path = path.clone();
    src_path.push("src/");

    let md_files = bismuth_md::load::load_from_dir(
        &src_path,
        &std::path::PathBuf::from(src_path.clone())
            .canonicalize()
            .unwrap(),
    )
    .unwrap();
    let tokenized_file = run_lexer(md_files);
    let parsers = run_parser(tokenized_file);

    if PathBuf::from("./build/").exists() {
        let mut check_remove_dir = YesNo::new(
            String::from("Would you like to proceed (y/n):"),
            String::from("Warning! All the contents in the ./build dir will be removed"),
            None,
        );
        check_remove_dir.run();
        if let Some(true) = check_remove_dir.result {
            println!("Removing dir...");
            std::fs::remove_dir_all(PathBuf::from("./build/")).unwrap();
        } else {
            println!("Exiting...");
            std::process::exit(1);
        }
    }

    println!("Rendering...");
    let renderers = render_list(parsers);
    println!("Writing files...");
    for r in renderers {
        r.write().unwrap();
    }
}
