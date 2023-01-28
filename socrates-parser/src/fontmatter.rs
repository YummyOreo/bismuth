use serde::Deserialize;
use serde_yaml::{from_str, Error};
use std::collections::BTreeMap;
use std::path::Path;

#[derive(Default, Deserialize)]
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
            if self.path.as_ref().expect("Should have a path") != &p {
                self.path = Some(p)
            }
        }

        if let Some(t) = updated_fm.title {
            if self.title.as_ref().expect("Should have a title") != &t {
                self.title = Some(t)
            }
        }

        let kind = updated_fm.kind.unwrap_or("default".to_string());
        if &kind == self.kind.as_ref().unwrap_or(&"".to_string()) {
            self.kind = Some(kind);
        }

        if self.values != updated_fm.values {
            self.values = updated_fm.values;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_yml() {}
}
