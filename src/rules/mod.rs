//! Built-in and custom rules

use crate::types::{BoxedRule, Rule};
use once_cell::sync::Lazy;

mod md009;
mod md010;
mod md012;
mod md041;
mod md047;

// TODO: Import more rules as they are implemented
// mod md001;
// mod md003;
// etc.

/// Global rule registry
pub static RULES: Lazy<Vec<BoxedRule>> = Lazy::new(|| {
    vec![
        Box::new(md009::MD009),
        Box::new(md010::MD010),
        Box::new(md012::MD012),
        Box::new(md041::MD041),
        Box::new(md047::MD047),
        // TODO: Add more rules here as they are implemented
        // Box::new(md001::MD001),
        // Box::new(md003::MD003),
    ]
});

/// Get all built-in rules
pub fn get_rules() -> &'static [BoxedRule] {
    &RULES
}

/// Find a rule by name
pub fn find_rule(name: &str) -> Option<&'static dyn Rule> {
    let name_upper = name.to_uppercase();
    RULES.iter().find_map(|rule| {
        if rule
            .names()
            .iter()
            .any(|n| n.to_uppercase() == name_upper)
        {
            Some(&**rule)
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rules_registry() {
        let rules = get_rules();
        // Will be empty until we implement rules
        assert_eq!(rules.len(), 0);
    }
}
