use crate::Custom;
use bismuth_parser::Parser;

pub trait Plugin: std::fmt::Debug {
    fn run(&mut self, target: &mut Parser, others: &[Option<&Parser>]);
    fn pre_load(&mut self, target: &Parser, custom: &Custom);
}
