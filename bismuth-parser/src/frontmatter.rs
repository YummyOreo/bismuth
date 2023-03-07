use serde::Deserialize;
use serde_yaml::{from_str, Error};
use std::collections::{BTreeMap, HashMap};
use std::path::Path;

#[derive(Default, Deserialize, Debug, Clone)]
pub struct FrontMatter {
    title: Option<String>,
    path: Option<String>,

    kind: Option<String>,

    values: Option<Vec<BTreeMap<String, String>>>,
}

impl FrontMatter {
    pub fn new(path: &Path) -> Self {
        let title = path
            .file_name()
            .expect("Should be a file")
            .to_string_lossy()
            .to_string();

        FrontMatter {
            title: Some(title),
            path: Some(path.to_string_lossy().to_string()),
            kind: Some(String::from("default")),
            ..Default::default()
        }
    }

    pub fn get_value(&self, key: &str) -> Option<&String> {
        self.values
            .as_ref()?
            .iter()
            .find(|t| t.contains_key(key))?
            .get(key)
    }

    pub fn get_values(&self) -> Option<HashMap<String, String>> {
        let mut hm: HashMap<String, String> = HashMap::new();
        for btm in &self.values.clone()? {
            let btm_vec = btm
                .iter()
                .map(|(s, b)| (s.to_owned(), b.to_owned()))
                .collect::<Vec<(String, String)>>();
            for (k, v) in btm_vec {
                hm.insert(k, v);
            }
        }
        Some(hm)
    }

    pub fn get_kind(&self) -> Option<&String> {
        self.kind.as_ref()
    }

    pub fn get_path(&self) -> Option<&String> {
        self.path.as_ref()
    }

    pub fn get_title(&self) -> Option<&String> {
        self.title.as_ref()
    }

    fn fill_defaults(&mut self) {
        if self.kind.is_none() {
            self.kind = Some(String::from("default"));
        }
    }

    pub fn update_from_str(&mut self, s: &str) -> Result<(), Error> {
        let updated_fm: FrontMatter = from_str(s)?;

        if let Some(p) = updated_fm.path {
            let p = Some(p);
            if self.path != p {
                self.path = p
            }
        }

        if let Some(t) = updated_fm.title {
            let t = Some(t);
            if self.title != t {
                self.title = t
            }
        }

        if let Some(k) = updated_fm.kind {
            let k = Some(k);
            if self.kind != k {
                self.kind = k
            }
        }

        if self.values != updated_fm.values {
            self.values = updated_fm.values;
        }

        self.fill_defaults();

        Ok(())
    }
}

#[cfg(test)]
mod tests_utils {
    use super::*;

    pub fn run_snapshot(mut fm: FrontMatter, update: &str) -> String {
        fm.update_from_str(update).unwrap();
        if let Some(mut values) = fm.values.clone() {
            values.sort();
            fm.values = Some(values);
        }
        format!("{fm:#?}")
    }

    macro_rules! snapshot {
        ($name:tt, $update:tt) => {
            #[test]
            fn $name() {
                let fm: FrontMatter = Default::default();
                let mut settings = insta::Settings::clone_current();
                settings.set_snapshot_path("../testdata/output/utils/");
                settings.bind(|| {
                    insta::assert_snapshot!(run_snapshot(fm, $update));
                });
            }
        };

        ($name:tt, $update:tt, $($key:tt, $value:expr),*) => {
            #[test]
            fn $name() {
                let fm = FrontMatter {
                    $(
                        $key: Some($value),
                    )*
                        ..Default::default()
                };
                let mut settings = insta::Settings::clone_current();
                settings.set_snapshot_path("../testdata/output/utils/");
                settings.bind(|| {
                    insta::assert_snapshot!(run_snapshot(fm, $update));
                });
            }
        };
    }

    snapshot!(
        test_load,
        "
        title: Test
        path: /test
        kind: test
        values:
            - test: te
        "
    );

    snapshot!(
        test_load_1,
        "
        title: this is a title
        kind: This is another test
        ",
        path,
        "/path/test".to_string()
    );

    snapshot!(
        test_load_2,
        "
        title: this is a title
        values:
            - value-1: test
            - value-2: test 2
        ",
        path,
        "/path/test".to_string()
    );
}
