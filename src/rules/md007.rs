//! MD007 - Unordered list indentation
//!
//! This rule checks that unordered list indentation is consistent.

use crate::parser::{Token, TokenExt};
use crate::types::{FixInfo, LintError, ParserType, Rule, RuleParams, Severity};
use std::collections::HashMap;

pub struct MD007;

impl Rule for MD007 {
    fn names(&self) -> &[&'static str] {
        &["MD007", "ul-indent"]
    }

    fn description(&self) -> &'static str {
        "Unordered list indentation"
    }

    fn tags(&self) -> &[&'static str] {
        &["bullet", "ul", "indentation"]
    }

    fn parser_type(&self) -> ParserType {
        ParserType::Micromark
    }

    fn information(&self) -> Option<&'static str> {
        Some("https://github.com/DavidAnson/markdownlint/blob/main/doc/md007.md")
    }

    fn lint(&self, params: &RuleParams) -> Vec<LintError> {
        let mut errors = Vec::new();

        // Get configuration
        let indent = params
            .config
            .get("indent")
            .and_then(|v| v.as_u64())
            .unwrap_or(2) as usize;

        let start_indented = params
            .config
            .get("start_indented")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let start_indent = params
            .config
            .get("start_indent")
            .and_then(|v| v.as_u64())
            .map(|v| v as usize)
            .unwrap_or(indent);

        // Build a map to track unordered list nesting levels
        let mut list_nesting_levels: HashMap<usize, usize> = HashMap::new();

        // First pass: identify all lists and their nesting levels
        for (idx, token) in params.tokens.iter().enumerate() {
            if token.token_type == "list" {
                // Check if it's an unordered list by examining the first list item
                if let Some(is_unordered) = is_unordered_list(params.tokens, token) {
                    if is_unordered {
                        let nesting = calculate_nesting_level(params.tokens, idx);
                        list_nesting_levels.insert(idx, nesting);
                    }
                }
            }
        }

        // Track the last blockquote to handle blockquote adjustments
        let mut last_blockquote_end_line = 0;
        let mut last_blockquote_end_column = 0;

        // Process all tokens
        for (idx, token) in params.tokens.iter().enumerate() {
            // Track blockquotes
            if token.token_type == "blockQuote" {
                last_blockquote_end_line = token.end_line;
                last_blockquote_end_column = token.end_column;
            }

            // Check list items
            if token.token_type == "listItem" {
                // Find the parent list
                if let Some(parent_idx) = token.parent {
                    if let Some(&nesting) = list_nesting_levels.get(&parent_idx) {
                        // This is a list item in an unordered list we care about
                        let line_number = token.start_line;

                        if line_number > params.lines.len() {
                            continue;
                        }

                        let line = &params.lines[line_number - 1];

                        // Calculate expected and actual indentation
                        let base_indent = if is_in_footnote(params.tokens, idx) {
                            4
                        } else {
                            0
                        };

                        let expected_indent = base_indent
                            + if start_indented { start_indent } else { 0 }
                            + (nesting * indent);

                        // Calculate actual indent (column - 1 since columns are 1-based)
                        let mut actual_indent = token.start_column - 1;

                        // Adjust for blockquotes on the same line
                        if last_blockquote_end_line == line_number {
                            actual_indent = actual_indent.saturating_sub(last_blockquote_end_column - 1);
                        }

                        // Report error if indentation doesn't match
                        if actual_indent != expected_indent {
                            let range_end = line.trim_end_matches('\n').trim_end_matches('\r').len();
                            let range = (1, range_end);

                            let fix_info = FixInfo {
                                line_number: None,
                                edit_column: Some(token.start_column - actual_indent),
                                delete_count: Some(actual_indent.saturating_sub(expected_indent) as i32),
                                insert_text: if expected_indent > actual_indent {
                                    Some(" ".repeat(expected_indent - actual_indent))
                                } else {
                                    None
                                },
                            };

                            errors.push(LintError {
                                line_number,
                                rule_names: self.names().iter().map(|s| s.to_string()).collect(),
                                rule_description: self.description().to_string(),
                                error_detail: Some(format!(
                                    "Expected: {}; Actual: {}",
                                    expected_indent, actual_indent
                                )),
                                error_context: None,
                                rule_information: self.information().map(|s| s.to_string()),
                                error_range: Some(range),
                                fix_info: Some(fix_info),
                                severity: Severity::Error,
                            });
                        }
                    }
                }
            }
        }

        errors
    }
}

/// Check if a list is unordered by examining its content
fn is_unordered_list(tokens: &[Token], list_token: &Token) -> Option<bool> {
    // Look for a list item child
    for child_idx in &list_token.children {
        if let Some(child) = tokens.get(*child_idx) {
            if child.token_type == "listItem" {
                // Check the line content to determine if it's ordered or unordered
                // Unordered lists use -, *, or +
                // Ordered lists use numbers followed by . or )
                // For now, we'll use a heuristic based on the token text
                // In a real implementation, we'd need access to the line content
                // or additional metadata from the parser

                // Since comrak's List node has a list_type field we should check,
                // but our Token abstraction doesn't expose it yet.
                // For now, assume all lists without explicit ordering are unordered.
                return Some(true);
            }
        }
    }
    None
}

/// Calculate the nesting level of an unordered list
/// (how many unordered lists are ancestors)
fn calculate_nesting_level(tokens: &[Token], token_idx: usize) -> usize {
    let mut nesting = 0;
    let mut current_idx = token_idx;

    while let Some(token) = tokens.get(current_idx) {
        if let Some(parent_idx) = token.parent {
            if let Some(parent) = tokens.get(parent_idx) {
                if parent.token_type == "list" {
                    // Check if parent is unordered
                    if let Some(true) = is_unordered_list(tokens, parent) {
                        nesting += 1;
                    } else {
                        // Parent is ordered list, don't count this nesting
                        return 0;
                    }
                }
                current_idx = parent_idx;
            } else {
                break;
            }
        } else {
            break;
        }
    }

    nesting
}

/// Check if a token is inside a footnote definition
fn is_in_footnote(tokens: &[Token], token_idx: usize) -> bool {
    let mut current_idx = token_idx;

    while let Some(token) = tokens.get(current_idx) {
        if token.token_type == "footnoteDefinition" {
            return true;
        }
        if let Some(parent_idx) = token.parent {
            current_idx = parent_idx;
        } else {
            break;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Token;
    use std::collections::HashMap;

    fn create_token(
        token_type: &str,
        start_line: usize,
        start_column: usize,
        parent: Option<usize>,
    ) -> Token {
        Token {
            token_type: token_type.to_string(),
            start_line,
            start_column,
            end_line: start_line,
            end_column: start_column + 1,
            text: String::new(),
            children: Vec::new(),
            parent,
        }
    }

    #[test]
    fn test_md007_correct_indentation() {
        let lines = vec![
            "- Item 1\n".to_string(),
            "  - Item 2\n".to_string(),
            "    - Item 3\n".to_string(),
        ];

        // Create token structure
        let mut tokens = vec![
            create_token("list", 1, 1, None),        // index 0
            create_token("listItem", 1, 1, Some(0)), // index 1
            create_token("list", 2, 3, Some(1)),     // index 2
            create_token("listItem", 2, 3, Some(2)), // index 3
            create_token("list", 3, 5, Some(3)),     // index 4
            create_token("listItem", 3, 5, Some(4)), // index 5
        ];

        // Set up children
        tokens[0].children = vec![1];
        tokens[1].children = vec![2];
        tokens[2].children = vec![3];
        tokens[3].children = vec![4];
        tokens[4].children = vec![5];

        let params = RuleParams {
            name: "test.md",
            version: "0.1.0",
            lines: &lines,
            front_matter_lines: &[],
            tokens: &tokens,
            config: &HashMap::new(),
        };

        let rule = MD007;
        let errors = rule.lint(&params);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn test_md007_incorrect_indentation() {
        let lines = vec![
            "- Item 1\n".to_string(),
            "    - Item 2\n".to_string(), // Should be 2 spaces, but is 4
        ];

        let mut tokens = vec![
            create_token("list", 1, 1, None),        // index 0
            create_token("listItem", 1, 1, Some(0)), // index 1
            create_token("list", 2, 5, Some(1)),     // index 2 - starts at column 5 (4 spaces + 1)
            create_token("listItem", 2, 5, Some(2)), // index 3
        ];

        tokens[0].children = vec![1];
        tokens[1].children = vec![2];
        tokens[2].children = vec![3];

        let params = RuleParams {
            name: "test.md",
            version: "0.1.0",
            lines: &lines,
            front_matter_lines: &[],
            tokens: &tokens,
            config: &HashMap::new(),
        };

        let rule = MD007;
        let errors = rule.lint(&params);

        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].line_number, 2);
        assert!(errors[0]
            .error_detail
            .as_ref()
            .unwrap()
            .contains("Expected: 2; Actual: 4"));
    }

    #[test]
    fn test_md007_custom_indent() {
        let lines = vec![
            "- Item 1\n".to_string(),
            "  - Item 2\n".to_string(), // 2 spaces, but config says 4
        ];

        let mut tokens = vec![
            create_token("list", 1, 1, None),
            create_token("listItem", 1, 1, Some(0)),
            create_token("list", 2, 3, Some(1)),
            create_token("listItem", 2, 3, Some(2)),
        ];

        tokens[0].children = vec![1];
        tokens[1].children = vec![2];
        tokens[2].children = vec![3];

        let mut config = HashMap::new();
        config.insert("indent".to_string(), serde_json::json!(4));

        let params = RuleParams {
            name: "test.md",
            version: "0.1.0",
            lines: &lines,
            front_matter_lines: &[],
            tokens: &tokens,
            config: &config,
        };

        let rule = MD007;
        let errors = rule.lint(&params);

        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].line_number, 2);
        assert!(errors[0]
            .error_detail
            .as_ref()
            .unwrap()
            .contains("Expected: 4; Actual: 2"));
    }

    #[test]
    fn test_md007_start_indented() {
        let lines = vec![
            "   - Item 1\n".to_string(), // Start indented by 3
            "      - Item 2\n".to_string(), // 3 + 3 = 6 spaces
        ];

        let mut tokens = vec![
            create_token("list", 1, 4, None),        // starts at column 4 (3 spaces + 1)
            create_token("listItem", 1, 4, Some(0)),
            create_token("list", 2, 7, Some(1)),     // starts at column 7 (6 spaces + 1)
            create_token("listItem", 2, 7, Some(2)),
        ];

        tokens[0].children = vec![1];
        tokens[1].children = vec![2];
        tokens[2].children = vec![3];

        let mut config = HashMap::new();
        config.insert("start_indented".to_string(), serde_json::json!(true));
        config.insert("indent".to_string(), serde_json::json!(3));

        let params = RuleParams {
            name: "test.md",
            version: "0.1.0",
            lines: &lines,
            front_matter_lines: &[],
            tokens: &tokens,
            config: &config,
        };

        let rule = MD007;
        let errors = rule.lint(&params);
        assert_eq!(errors.len(), 0);
    }
}
