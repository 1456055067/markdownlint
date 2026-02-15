//! MD028 - Blank line inside blockquote

use crate::types::{LintError, ParserType, Rule, RuleParams, Severity};

pub struct MD028;

impl Rule for MD028 {
    fn names(&self) -> &[&'static str] {
        &["MD028", "no-blanks-blockquote"]
    }

    fn description(&self) -> &'static str {
        "Blank line inside blockquote"
    }

    fn tags(&self) -> &[&'static str] {
        &["blockquote", "whitespace"]
    }

    fn parser_type(&self) -> ParserType {
        ParserType::None
    }

    fn information(&self) -> Option<&'static str> {
        Some("https://github.com/DavidAnson/markdownlint/blob/main/doc/md028.md")
    }

    fn lint(&self, params: &RuleParams) -> Vec<LintError> {
        let mut errors = Vec::new();
        let mut in_blockquote = false;
        let mut blank_line = 0;

        for (idx, line) in params.lines.iter().enumerate() {
            let line_number = idx + 1;
            let trimmed = line.trim();

            if trimmed.starts_with('>') {
                if blank_line > 0 && in_blockquote {
                    errors.push(LintError {
                        line_number: blank_line,
                        rule_names: self.names().iter().map(|s| s.to_string()).collect(),
                        rule_description: self.description().to_string(),
                        error_detail: None,
                        error_context: None,
                        rule_information: self.information().map(|s| s.to_string()),
                        error_range: None,
                        fix_info: None,
                        severity: Severity::Error,
                    });
                }
                in_blockquote = true;
                blank_line = 0;
            } else if trimmed.is_empty() {
                if in_blockquote {
                    blank_line = line_number;
                }
            } else {
                in_blockquote = false;
                blank_line = 0;
            }
        }

        errors
    }
}
