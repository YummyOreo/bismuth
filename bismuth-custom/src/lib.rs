#![allow(dead_code)]

use bismuth_parser::{Parser, Metadata, tree::{Element, Ast, Kind}};

fn get_customs(ast: &Ast) -> Vec<&Element> {
    let mut elements: Vec<&Element> = ast.elements.iter().collect();
    elements.retain(|&e| matches!(e.kind, Kind::CustomElement(_)));
    elements
}

pub fn parse_custom(mut target: Parser, others: Vec<&Parser>) -> Parser {
    let custom_elms: Vec<&Element> = get_customs(&target.ast);
    todo!()
}
