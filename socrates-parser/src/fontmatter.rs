use serde::Deserialize;
use serde_yaml::{from_str, Error};
use std::collections::BTreeMap;
use std::path::Path;

#[derive(Default, Deserialize, Debug)]
pub struct FontMatter {
    title: Option<String>,
    path: Option<String>,

    kind: Option<String>,

    values: Option<Vec<BTreeMap<String, String>>>,
}

impl FontMatter {
    pub fn new(path: &Path) -> Self {
        let title = path
            .file_name()
            .expect("Should be a file")
            .to_string_lossy()
            .to_string();

        FontMatter {
            title: Some(title),
            path: Some(path.to_string_lossy().to_string()),
            ..Default::default()
        }
    }

    pub fn update_from_str(&mut self, s: &str) -> Result<(), Error> {
        let updated_fm: FontMatter = from_str(s)?;

        if let Some(p) = updated_fm.path {
            if self.path.as_ref().unwrap_or(&"".to_string()) != &p {
                self.path = Some(p)
            }
        }

        if let Some(t) = updated_fm.title {
            if self.title.as_ref().unwrap_or(&"".to_string()) != &t {
                self.title = Some(t)
            }
        }

        let kind = updated_fm.kind.unwrap_or("default".to_string());
        if &kind != self.kind.as_ref().unwrap_or(&"".to_string()) {
            self.kind = Some(kind);
        }

        if self.values != updated_fm.values {
            self.values = updated_fm.values;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests_utils {
    use super::*;

    macro_rules! snapshot {
        ($string:tt) => {
            let mut settings = insta::Settings::clone_current();
            settings.set_snapshot_path("../testdata/output/utils/");
            settings.bind(|| {
                insta::assert_snapshot!(format!("{:#?}", $string));
            });
        };
    }

    #[test]
    fn test_load() {
        let mut fm = FontMatter::default();
        let s_fm = "title: Test\npath: /test\nkind: test\nvalues:\n    - test: te";
        fm.update_from_str(s_fm).unwrap();

        snapshot!(fm);
    }

    #[test]
    fn test_load_1() {
        let mut fm = FontMatter {
            path: Some("/path/test".to_string()),
            ..Default::default()
        };
        let s_fm = "title: this is a title\nkind: This is another test";
        fm.update_from_str(s_fm).unwrap();

        snapshot!(fm);
    }

    #[test]
    fn test_load_2() {
        let mut fm = FontMatter {
            path: Some("/path/test".to_string()),
            ..Default::default()
        };
        let s_fm = "title: this is a title\nvalues:\n    - value-1: test\n    - value-2: test 2";
        fm.update_from_str(s_fm).unwrap();

        snapshot!(fm);
    }
}
