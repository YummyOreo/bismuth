#![allow(dead_code)]

use bismuth_parser::{
    tree::{Ast, Element, Kind},
    Metadata, Parser,
};

pub struct Custom {
    // todo
}

fn get_customs(ast: &Ast) -> Vec<&Element> {
    let mut elements: Vec<&Element> = ast.elements.iter().collect();
    elements.retain(|&e| matches!(e.kind, Kind::CustomElement(_)));
    elements
}

pub fn parse_custom(mut target: Parser, others: Vec<&Parser>) -> Parser {
    let custom_elms: Vec<&Element> = get_customs(&target.ast);
    todo!()
}

#[cfg(test)]
mod test_utils {
    use super::*;

    macro_rules! snapshot {
        ($content:tt) => {
            let mut settings = insta::Settings::clone_current();
            settings.set_snapshot_path("../testdata/output/");
            settings.bind(|| {
                insta::assert_snapshot!($content);
            });
        };
    }
    #[test]
    fn get_customs_test() {
        let mut parser =
            bismuth_parser::Parser::new_test("/test/", "%{{\nname: test\nother: key\n}}");
        parser.parse().unwrap();

        let customs = format!("{:#?}", get_customs(&parser.ast));
        snapshot!(customs);
    }
}
