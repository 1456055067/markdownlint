//! MD019 - Multiple spaces after hash on atx style heading

use crate::types::{FixInfo, LintError, ParserType, Rule, RuleParams, Severity};

pub struct MD019;

impl Rule for MD019 {
    fn names(&self) -> &[&'static str] {
        &["MD019", "no-multiple-space-atx"]
    }

    fn description(&self) -> &'static str {
        "Multiple spaces after hash on atx style heading"
    }

    fn tags(&self) -> &[&'static str] {
        &["headings", "atx", "spaces"]
    }

    fn parser_type(&self) -> ParserType {
        ParserType::None
    }

    fn information(&self) -> Option<&'static str> {
        Some("https://github.com/DavidAnson/markdownlint/blob/main/doc/md019.md")
    }

    fn lint(&self, params: &RuleParams) -> Vec<LintError> {
        let mut errors = Vec::new();

        for (idx, line) in params.lines.iter().enumerate() {
            let line_number = idx + 1;
            let trimmed = line.trim_start();

            if trimmed.starts_with('#') {
                let hash_count = trimmed.chars().take_while(|&c| c == '#').count();
                if hash_count > 0 && hash_count <= 6 {
                    let after_hash = &trimmed[hash_count..];
                    let space_count = after_hash.chars().take_while(|&c| c == ' ').count();

                    if space_count > 1 {
                        errors.push(LintError {
                            line_number,
                            rule_names: self.names().iter().map(|s| s.to_string()).collect(),
                            rule_description: self.description().to_string(),
                            error_detail: Some(format!("Expected: 1; Actual: {}", space_count)),
                            error_context: None,
                            rule_information: self.information().map(|s| s.to_string()),
                            error_range: Some((hash_count + 2, space_count - 1)),
                            fix_info: Some(FixInfo {
                                line_number: None,
                                edit_column: Some(hash_count + 2),
                                delete_count: Some((space_count - 1) as i32),
                                insert_text: None,
                            }),
                            severity: Severity::Error,
                        });
                    }
                }
            }
        }

        errors
    }
}
