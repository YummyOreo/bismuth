use bismuth_html::render_one;
use bismuth_parser::Parser;

use regex::Regex;

pub fn run_snapshot_str(content: &str, frontmatter: Option<String>) -> String {
    let content = format_expected(&frontmatter.unwrap_or_default()) + content;

    println!("input: '\n{content}\n'");
    println!("");
    println!("---");
    println!("");

    let mut parser = Parser::new_test("/test/", &content);
    parser.parse().unwrap();
    let res = render_one(parser).unwrap();

    // skips annoying eol spaces
    let rg = Regex::new(r"\s\n").expect("Is valid regex");
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

macro_rules! snapshot_str {
    ($name:tt, $content:tt, $expected:tt) => {
        #[test]
        fn $name() {
            let result = run_snapshot_str($content, None);
            let expect = format_expected($expected);
            assert_eq!(result, expect)
        }
    };
    ($name:tt, $content:tt, $frontmatter:tt, $expected:tt) => {
        #[test]
        fn $name() {
            let result = run_snapshot_str($content, Some($frontmatter.to_string()));
            let expect = format_expected($expected);
            println!("result: '\n{result}\n'");
            println!("");
            println!("---");
            println!("");
            println!("expected: '\n{expect}\n'");
            assert_eq!(result, expect)
        }
    };
}

pub(crate) use snapshot_str;
