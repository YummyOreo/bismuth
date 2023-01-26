use serde::Deserialize;
use serde_yaml::from_str;
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
    pub fn new(path: &Path, title: Option<String>) -> Self {
        let title = match title {
            Some(t) => t,
            None => path
                .file_name()
                .expect("Should be a file")
                .to_string_lossy()
                .to_string(),
        };

        FontMatter {
            title: Some(title),
            path: Some(path.to_string_lossy().to_string()),
            ..Default::default()
        }
    }

    pub fn from_str(s: &str) {}
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_yml() {
        use super::*;
        FontMatter::from_str("");
    }
}
