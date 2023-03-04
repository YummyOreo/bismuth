use crate::render::Render;

pub struct Template {}

impl Template {
    pub fn new() -> Self {
        Self {}
    }
}

impl Render for Template {
    fn render(&mut self) -> String {
        todo!()
    }
}
