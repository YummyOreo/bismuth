#[derive(Default, Debug)]
pub struct Template {
    pub content: String,
}

impl Template {
    pub fn new(content: String) -> Self {
        Template { content }
    }
}
