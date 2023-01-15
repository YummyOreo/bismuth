use std::path::Path;
use socrates_md::MarkdownFileError;

mod arguments;
mod config;

fn get_files(path: &Path) -> Vec<socrates_md::MarkdownFile> {
    let config = config::Config::new(path);
    match socrates_md::load::load_from_dir(&config.directory.to_path_buf()) {
        Ok(files) => files,
        Err(e) => match e {
            MarkdownFileError::IsFileError(_) | MarkdownFileError::NotDirectoryError(_) => {
                socrates_md::load::load_from_dir(&socrates_error::path::md_file_error("").unwrap())
                    .unwrap()
            }
            _ => {
                panic!("{:#?}", e)
            }
        },
    }
}

fn main() {
    let str_args = arguments::get_str_args();
    let args = arguments::parse_args(&str_args);

    let path = {
        match args
            .iter()
            .find(|p| matches!(p, arguments::Args::Dir(_)))
            .expect("There should be a dir")
        {
            arguments::Args::Dir(s) => s,
            _ => panic!("Something has gone wrong"),
        }
    };

    let md_files = get_files(path);

    println!("{:?}", md_files);
}
