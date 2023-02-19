use serde_yaml::{from_str, Error};
use std::collections::{BTreeMap, HashMap};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CustomElmError {
    #[error("Error parsing yaml {0}")]
    YamlParse(Error),
    #[error("no keys")]
    NoKeys,
    #[error("no name")]
    NoName,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct CustomElm {
    pub name: String,
    pub values: HashMap<String, String>,
    pub body: Option<String>,
    pub template: Option<String>,
}

impl CustomElm {
    pub fn new() -> Self {
        CustomElm {
            ..Default::default()
        }
    }

    pub fn from_string(s: &str) -> Result<Self, CustomElmError> {
        let sections = s.splitn(2, "---\n").collect::<Vec<&str>>();
        println!("{sections:?}");
        println!("{s:?}");
        let yaml = sections.first().ok_or(CustomElmError::NoKeys)?;
        println!("{yaml}");
        let body = sections.get(1).map(|p| p.to_string());

        let mut parsed_yaml: BTreeMap<String, String> =
            from_str(yaml).map_err(CustomElmError::YamlParse)?;

        let name_pos = parsed_yaml
            .keys()
            .position(|p| p.to_lowercase() == "name")
            .ok_or(CustomElmError::NoName)?;
        let name_key = parsed_yaml
            .keys()
            .nth(name_pos)
            .expect("Should have the key at the position")
            .clone();
        let name = parsed_yaml
            .get(&name_key)
            .expect("Should be able to get the key")
            .to_string();

        parsed_yaml.remove(&name_key);

        let mut values: HashMap<String, String> = HashMap::new();
        for (key, value) in parsed_yaml {
            let _ = values.insert(key, value);
        }

        Ok(CustomElm {
            name,
            values,
            body,
            template: None,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn snapshot(content: &str) -> String {
        let elm = CustomElm::from_string(content).unwrap();
        let mut values: Vec<(&String, &String)> = elm.values.iter().collect();
        values.sort();

        let values_str = {
            let mut s = String::from("{\n");
            for (key, value) in values {
                s.push_str(&format!("\t\t{key}, {value}\n"));
            }
            s.push_str("\t},");
            s
        };

        format!(
            "{{\n\tname: {:#?}\n\tvalues: {values_str}\n\tbody: {}\n}}",
            elm.name,
            {
                match elm.body {
                    Some(body) => {
                        format!("{body:#?}")
                    }
                    None => "None".to_string(),
                }
            }
        )
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
