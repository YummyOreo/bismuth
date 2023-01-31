use serde_yaml::from_str;
use std::collections::{BTreeMap, HashMap};

pub enum CustomElmError {}

#[derive(Default, Debug)]
pub struct CustomElm {
    name: String,
    values: HashMap<String, String>,
    body: Option<String>,
}

impl CustomElm {
    pub fn new() -> Self {
        CustomElm {
            ..Default::default()
        }
    }

    pub fn from_string(s: &str) -> Option<Self> {
        let sections = s.split("---\n").collect::<Vec<&str>>();
        let yaml = sections.first()?;
        let body = sections.get(1).map(|p| p.to_string());

        let mut parsed_yaml: BTreeMap<String, String> = from_str(yaml).unwrap();

        let name_pos = parsed_yaml
            .keys()
            .position(|p| p.to_lowercase() == "name")?;
        let name_key = parsed_yaml.keys().nth(name_pos)?.clone();
        let name = parsed_yaml.get(&name_key)?.to_string();

        parsed_yaml.remove(&name_key);

        let mut values: HashMap<String, String> = HashMap::new();
        for (key, value) in parsed_yaml {
            values.insert(key, value)?;
        }

        Some(CustomElm { name, values, body })
    }
}
