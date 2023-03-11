#![allow(dead_code, unused)]

use bismuth_custom::parse_custom;
use bismuth_html::render_one;
use bismuth_parser::Parser;

use regex::Regex;
use std::{fs, path::PathBuf};

pub fn run_snapshot_str(content: &str, frontmatter: Option<String>, path: &str) -> String {
    let content = format_expected(&frontmatter.unwrap_or_default()) + content;

    println!("input: '\n{content}\n'");
    println!("");
    println!("---");
    println!("");

    let mut parser = Parser::new_test(path, &content);
    parser.parse().unwrap();
    let parser = parse_custom(parser, &[]);
    let res = render_one(parser).unwrap();
    println!("result: \n'{res}\n'");

    // skips annoying eol spaces
    let rg = Regex::new(r"[ \t\r]+\n").expect("Is valid regex");
    rg.replace_all(&res, "\n").to_string()
}

pub fn format_expected(content: &str) -> String {
    let mut content = content.to_string();
    if content.starts_with('\n') {
        let mut chars = content.chars();
        chars.next();
        content = chars.collect::<String>();
    }
    content.to_string()
}

pub fn run_snapshot_path(path_str: &str, expected: &str) -> String {
    let path = PathBuf::from(path_str);
    let content = fs::read_to_string(&path).unwrap();
    let res = run_snapshot_str(&content, None, path_str);

    // FOR DEBUGGING: FIND A BETTER SOLUTION
    // let output_file = PathBuf::from("./test.html");
    // fs::write(output_file, res.clone()).unwrap();
    // let output_file = PathBuf::from("./test_expected.html");
    // fs::write(output_file, expected.clone()).unwrap();
    res
}

macro_rules! snapshot_str {
    ($name:tt, $content:tt, $expected:tt) => {
        #[test]
        fn $name() {
            let result = run_snapshot_str($content, None, "/test/");
            let expect = format_expected($expected);
            assert_eq!(result, expect)
        }
    };
    ($name:tt, $content:tt, $frontmatter:tt, $expected:tt) => {
        #[test]
        fn $name() {
            let result = run_snapshot_str($content, Some($frontmatter.to_string()), "/test/");
            let expect = format_expected($expected);
            println!("");
            println!("---");
            println!("");
            println!("expected: '\n{expect}\n'");
            assert_eq!(result, expect)
        }
    };
}

macro_rules! snapshot_path {
    ($name:tt, $path:tt, $expected:tt) => {
        #[test]
        fn $name() {
            let result = run_snapshot_path($path, $expected);
            let expect = format_expected($expected);
            assert_eq!(result, expect)
        }
    };
}

pub(crate) use snapshot_path;
pub(crate) use snapshot_str;
