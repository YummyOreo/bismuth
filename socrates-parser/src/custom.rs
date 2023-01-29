use std::collections::{BTreeMap, HashMap};

#[derive(Default, Debug)]
pub struct CustomElm {
    name: String,
    vaues: HashMap<String, String>,
    body: Option<String>,
}

impl CustomElm {}
