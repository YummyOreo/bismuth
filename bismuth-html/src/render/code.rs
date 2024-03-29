use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

use syntect::Error;
use thiserror::Error;

fn init() -> (SyntaxSet, ThemeSet) {
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    (ps, ts)
}

#[derive(Debug, Error)]
pub enum HighlightError {
    #[error("Internal error {0}")]
    Internal(#[from] Error),
}

pub fn highlight(lang: String, code: String) -> Result<String, HighlightError> {
    let code = code.replacen('\n', "", 1);
    let (ps, ts) = init();
    let syntax = ps
        .find_syntax_by_token(&lang)
        .unwrap_or_else(|| ps.find_syntax_plain_text());

    highlighted_html_for_string(&code, &ps, syntax, &ts.themes["base16-mocha.dark"])
        .map_err(HighlightError::Internal)
}

#[cfg(test)]
mod test {

    use super::*;
    macro_rules! snapshot {
        ($content:tt) => {
            let mut settings = insta::Settings::clone_current();
            settings.set_snapshot_path("../../testdata/output/render/code/");
            settings.bind(|| {
                insta::assert_snapshot!($content);
            });
        };
    }

    #[test]
    fn test() {
        let code = "fn test() {\n\tlet a = 10;\n}".to_string();
        let html = highlight("rs".to_string(), code).unwrap();
        snapshot!(html);
    }

    #[test]
    fn test_inline() {
        let code = "let test = 10".to_string();
        let html = highlight("rs".to_string(), code).unwrap();
        snapshot!(html);
    }
}
