use std::env;
use std::path::Path;

pub fn get_str_args() -> Vec<String> {
    // skips the first because the first will be the file
    env::args().skip(1).collect()
}

#[derive(Debug, PartialEq)]
pub enum Args<'a> {
    Help,
    Dir(&'a Path),
}

pub fn parse_args(str_args: &[String]) -> Vec<Args> {
    let mut res = vec![];
    for (i, el) in str_args.iter().enumerate() {
        match el.as_str() {
            "--help" | "-h" => res.push(Args::Help),
            "--dir" | "-d" => res.push(Args::Dir(Path::new(
                str_args
                    .get(i + 1)
                    .unwrap_or_else(|| panic!("Please provide a directory.")),
            ))),
            _ => {}
        }
    }
    // checks if they have supplied a dir because if they have, then we may not get a correct dir
    // at the end of the arguments
    let has_dir = res.iter().any(|s| matches!(s, Args::Dir(_)));

    // gets the last argument
    let last = match str_args.last() {
        Some(s) => s,
        None => "./",
    };
    if !has_dir && !last.starts_with("--") {
        res.push(Args::Dir(Path::new(last)));
    }

    res
}

#[cfg(test)]
mod tests {
    use crate::arguments::{parse_args, Args};
    use std::path::Path;

    #[test]
    fn parse_args_no_panic() {
        let str_args = vec!["--dir".to_string(), ".\\test\\".to_string()];
        let should = vec![Args::Dir(Path::new("./test/"))];
        assert_eq!(should, parse_args(&str_args));

        let str_args = vec![".\\test\\".to_string()];
        let should = vec![Args::Dir(Path::new("./test/"))];
        assert_eq!(should, parse_args(&str_args));

        let str_args = vec![];
        let should = vec![Args::Dir(Path::new("./"))];
        assert_eq!(should, parse_args(&str_args));

        let str_args = vec!["--help".to_string()];
        let should = vec![Args::Help];
        assert_eq!(should, parse_args(&str_args));
    }

    #[test]
    #[should_panic(expected = "Please provide a directory.")]
    fn parse_args_panic() {
        let str_args = vec!["-d".to_string()];
        parse_args(&str_args);
    }
}
