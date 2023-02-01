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
        let sections = s.splitn(2, "---\n").collect::<Vec<&str>>();
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
            let _ = values.insert(key, value);
        }

        Some(CustomElm { name, values, body })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn snapshot(content: &str) -> String {
        format!("{:#?}", CustomElm::from_string(content).unwrap())
    }

    macro_rules! snapshot_load {
        ($name:tt, $content:tt) => {
            #[test]
            fn $name() {
                let mut settings = insta::Settings::clone_current();
                settings.set_snapshot_path("../testdata/output/utils");
                settings.bind(|| {
                    insta::assert_snapshot!(snapshot($content));
                });
            }
        };
    }

    snapshot_load!(test_load, "name: me\nvalue: not a key");
    snapshot_load!(
        test_load_1,
        "name: me\nvalue: not a key\n---\nthis is the body"
    );
    snapshot_load!(
        test_load_2,
        "name: this is a test name with --- dashes\nkey: value\ntext: this is a test \\n\n---\n<p>this is the body text</p>"
    );
}
