pub struct Rule {
    pub rule_type: RuleType,
    pub rule_content: String,
}

pub enum RuleType {
    Domain,
    Full,
    Keyword,
    Regex,
}

impl Rule {
    pub fn new(rule_type: RuleType, rule_content: String) -> Self {
        Rule {
            rule_type,
            rule_content,
        }
    }
}

impl PartialEq for Rule {
    fn eq(&self, other: &Self) -> bool {
        self.rule_type == other.rule_type && self.rule_content == other.rule_content
    }
}

impl PartialEq for RuleType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (RuleType::Domain, RuleType::Domain)
            | (RuleType::Full, RuleType::Full)
            | (RuleType::Keyword, RuleType::Keyword)
            | (RuleType::Regex, RuleType::Regex) => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.rule_type, self.rule_content)
    }
}

impl std::fmt::Display for RuleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuleType::Domain => write!(f, "domain"),
            RuleType::Full => write!(f, "full"),
            RuleType::Keyword => write!(f, "keyword"),
            RuleType::Regex => write!(f, "regex"),
        }
    }
}
