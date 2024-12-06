pub mod config;
pub mod download;
pub mod request;
pub mod rule;
pub mod tools;

#[cfg(test)]
mod tests {
    use super::rule::{Rule, RuleType};

    #[test]
    fn test_merge() {
        let rules_vec = vec![
            vec![
                Rule::new(RuleType::Domain, "a.b.c".to_string()),
                Rule::new(RuleType::Domain, "b.c".to_string()),
                Rule::new(RuleType::Domain, "c".to_string()),
                Rule::new(RuleType::Full, "a.b.c.test.com".to_string()),
                Rule::new(RuleType::Full, "1.a.test.com".to_string()),
                Rule::new(RuleType::Full, "c.test.com".to_string()),
                Rule::new(RuleType::Full, "test.com".to_string()),
            ],
            vec![
                Rule::new(RuleType::Domain, "a.b.c".to_string()),
                Rule::new(RuleType::Domain, "b.c".to_string()),
                Rule::new(RuleType::Domain, "c".to_string()),
                Rule::new(RuleType::Domain, "c.test.com".to_string()),
            ],
        ];
        let rules = super::tools::merge_and_remove_duplicates(rules_vec);
        for rule in &rules {
            println!("{}", rule);
        }
        assert_eq!(rules.len(), 4);
    }
}
