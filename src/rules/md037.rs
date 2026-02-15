//! MD037 - Spaces inside emphasis markers

use crate::types::{LintError, ParserType, Rule, RuleParams, Severity};
use regex::Regex;
use once_cell::sync::Lazy;

static EMPHASIS_SPACE_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(\*|_)( +[^*_]+?[^ *_]+ +)(\*|_)").unwrap()
});

pub struct MD037;

impl Rule for MD037 {
    fn names(&self) -> &[&'static str] {
        &["MD037", "no-space-in-emphasis"]
    }

    fn description(&self) -> &'static str {
        "Spaces inside emphasis markers"
    }

    fn tags(&self) -> &[&'static str] {
        &["whitespace", "emphasis"]
    }

    fn parser_type(&self) -> ParserType {
        ParserType::None
    }

    fn information(&self) -> Option<&'static str> {
        Some("https://github.com/DavidAnson/markdownlint/blob/main/doc/md037.md")
    }

    fn lint(&self, params: &RuleParams) -> Vec<LintError> {
        let mut errors = Vec::new();

        for (idx, line) in params.lines.iter().enumerate() {
            let line_number = idx + 1;

            for mat in EMPHASIS_SPACE_RE.find_iter(line) {
                errors.push(LintError {
                    line_number,
                    rule_names: self.names().iter().map(|s| s.to_string()).collect(),
                    rule_description: self.description().to_string(),
                    error_detail: None,
                    error_context: Some(mat.as_str().to_string()),
                    rule_information: self.information().map(|s| s.to_string()),
                    error_range: Some((mat.start() + 1, mat.len())),
                    fix_info: None,
                    severity: Severity::Error,
                });
            }
        }

        errors
    }
}
