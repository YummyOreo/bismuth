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

pub fn run_parser(files: Vec<bismuth_lexer::Lexer>) -> Vec<bismuth_parser::Parser> {
    let mut parsed_files_pre: Vec<bismuth_parser::Parser> = vec![];
    let mut parsed_files_post: Vec<bismuth_parser::Parser> = vec![];
    for file in files {
        let mut parser = bismuth_parser::Parser::new(file);
        parser.parse().unwrap();
        parsed_files_pre.push(parser)
    }
    for parser in parsed_files_pre.clone() {
        let parser = bismuth_custom::parse_custom(
            parser,
            &parsed_files_pre
                .iter()
                .collect::<Vec<&bismuth_parser::Parser>>(),
        );
        parsed_files_post.push(parser)
    }
    parsed_files_post
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
