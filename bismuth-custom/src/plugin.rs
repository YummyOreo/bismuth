use bismuth_parser::Parser;

pub trait Plugin {
    fn run(&mut self, target: &mut Parser, others: Vec<&Parser>);
    fn pre_load(&mut self, target: &Parser);
}
