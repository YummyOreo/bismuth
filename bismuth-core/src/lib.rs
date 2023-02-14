use std::path::Path;

mod arguments;
pub mod config;
// TODO: change this to remove --dir and just take a dir. Also, make it so there has to be a
// bismuth.toml file
pub fn get_files(path: &Path) -> Vec<bismuth_md::MarkdownFile> {
    match bismuth_md::load::load_from_dir(&path.to_path_buf()) {
        Ok(files) => files,
        Err(e) => panic!("{e:#?}"),
    }
}

pub fn run_lexer(files: Vec<bismuth_md::MarkdownFile>) -> Vec<bismuth_lexer::Lexer> {
    let mut lexer_files: Vec<bismuth_lexer::Lexer> = vec![];
    for file in files {
        let mut lexer = bismuth_lexer::Lexer::new(file);
        lexer.run_lexer().unwrap();
        lexer_files.push(lexer);
    }
    lexer_files
}

pub fn run(dir: String) {
    let path = Path::new(&dir).canonicalize().unwrap();

    let _config = config::Config::new(&path);

    let md_files = get_files(&path);
    println!("{:#?}", run_lexer(md_files));
}

pub fn entry(dir: String) {
    let args = arguments::parse_args();

    match args.command {
        arguments::Commands::Run => run(dir),
    }
}
