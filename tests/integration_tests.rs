//! Integration tests for markdownlint

use markdownlint::{lint_sync, Config, LintOptions};
use std::collections::HashMap;

#[test]
fn test_basic_lint_string() {
    let markdown = "# Hello World\n\nThis is a test.\n";
    let mut strings = HashMap::new();
    strings.insert("test.md".to_string(), markdown.to_string());

    let options = LintOptions {
        strings,
        ..Default::default()
    };

    let results = lint_sync(&options).unwrap();

    // Should complete without error (even if no rules are implemented yet)
    assert!(results.get("test.md").is_some());
}

#[test]
fn test_lint_with_config() {
    let markdown = "# Heading\n";
    let mut strings = HashMap::new();
    strings.insert("test.md".to_string(), markdown.to_string());

    let config = Config::new();

    let options = LintOptions {
        strings,
        config: Some(config),
        ..Default::default()
    };

    let results = lint_sync(&options).unwrap();
    assert!(results.get("test.md").is_some());
}

#[test]
fn test_results_display() {
    let results = markdownlint::LintResults::new();
    let display = format!("{}", results);
    assert_eq!(display, "");
}

#[test]
fn test_config_json_parsing() {
    let json = r#"{"default": true, "MD001": false}"#;
    let config: Config = serde_json::from_str(json).unwrap();

    assert_eq!(config.default, Some(true));
    assert!(!config.is_rule_enabled("MD001"));
    assert!(config.is_rule_enabled("MD003")); // Should use default
}

#[test]
fn test_library_version() {
    let version = markdownlint::version();
    assert!(!version.is_empty());
    assert!(version.starts_with("0."));
}
