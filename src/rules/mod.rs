//! Built-in and custom rules

use crate::types::{BoxedRule, Rule};
use once_cell::sync::Lazy;

// Implemented rules
mod md001;
mod md003;
mod md004;
mod md005;
mod md007;
mod md009;
mod md010;
mod md011;
mod md012;
mod md014;
mod md018;
mod md019;
mod md020;
mod md021;
mod md023;
mod md026;
mod md027;
mod md028;
mod md034;
mod md037;
mod md038;
mod md039;
mod md040;
mod md041;
mod md047;

// TODO: Remaining rules to implement
// md013, md022, md024, md025, md029, md030, md031, md032, md033, md035, md036,
// md042, md043, md044, md045, md046, md048, md049, md050, md051, md052, md053,
// md054, md055, md056, md058, md059, md060

/// Global rule registry
pub static RULES: Lazy<Vec<BoxedRule>> = Lazy::new(|| {
    vec![
        Box::new(md001::MD001),
        Box::new(md003::MD003),
        Box::new(md004::MD004),
        Box::new(md005::MD005),
        Box::new(md007::MD007),
        Box::new(md009::MD009),
        Box::new(md010::MD010),
        Box::new(md011::MD011),
        Box::new(md012::MD012),
        Box::new(md014::MD014),
        Box::new(md018::MD018),
        Box::new(md019::MD019),
        Box::new(md020::MD020),
        Box::new(md021::MD021),
        Box::new(md023::MD023),
        Box::new(md026::MD026),
        Box::new(md027::MD027),
        Box::new(md028::MD028),
        Box::new(md034::MD034),
        Box::new(md037::MD037),
        Box::new(md038::MD038),
        Box::new(md039::MD039),
        Box::new(md040::MD040),
        Box::new(md041::MD041),
        Box::new(md047::MD047),
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
        assert_eq!(rules.len(), 25, "Should have 25 rules registered");
    }

    #[test]
    fn test_find_rule_by_id() {
        assert!(find_rule("MD001").is_some());
        assert!(find_rule("MD009").is_some());
        assert!(find_rule("MD047").is_some());
    }

    #[test]
    fn test_find_rule_by_alias() {
        assert!(find_rule("no-trailing-spaces").is_some());
        assert!(find_rule("no-hard-tabs").is_some());
    }
}
