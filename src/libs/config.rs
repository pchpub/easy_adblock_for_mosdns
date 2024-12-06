use crate::libs::request::{RequestMethod, RequestStructure};
use lazy_static::lazy_static;
use std::io::{BufRead, Read};

lazy_static!(
    static ref HTTP_UA: String = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3".to_string();
);

use super::{
    download::Update,
    rule::{Rule, RuleType},
};

pub struct Config {
    pub rule_src: Vec<RuleSrc>,
}

pub struct RuleSrc {
    pub src_type: RuleSrcType,
    pub auto_update: bool,
}

pub enum RuleSrcType {
    File(String),            // file_path
    AdguardHomeRule(String), // URL
    Geosite(String, String), // geosite_update_url, geosite_category
    Unknown,
}

impl Update for RuleSrcType {
    async fn get(&self) -> Result<Vec<Rule>, String> {
        match self {
            RuleSrcType::File(file_path) => {
                let mut rules = vec![];
                // read file
                let file = std::fs::File::open(file_path);
                if file.is_err() {
                    return Err("Failed to open file".to_string());
                }
                let file = file.unwrap();
                let mut reader = std::io::BufReader::new(file);
                let mut buf = String::new();
                reader.read_to_string(&mut buf).unwrap();
                for line in buf.lines() {
                    let rule_string = line.trim().split_once(':').unwrap();
                    let rule_type = match rule_string.0.trim() {
                        "domain" => RuleType::Domain,
                        "full" => RuleType::Full,
                        _ => continue,
                    };
                    let rule_content = rule_string.1.trim().to_string();
                    rules.push(Rule::new(rule_type, rule_content));
                }

                Ok(rules)
            }
            RuleSrcType::AdguardHomeRule(url) => {
                let request_structure = RequestStructure::new(
                    RequestMethod::GET,
                    url.to_string(),
                    "".to_owned(),
                    None,
                    None,
                    Some(HTTP_UA.to_owned()),
                    None,
                );
                let response = request_structure.execute().await;
                if response.is_err() {
                    return Err("Failed to get AdguardHomeRule".to_string());
                }
                let response = response.unwrap();
                let content = response.2;
                let mut rules = vec![];
                for line in content.lines() {
                    let rule_string = line.trim().split_once(':').unwrap();
                    let rule_type = match rule_string.0.trim() {
                        "domain" => RuleType::Domain,
                        "full" => RuleType::Full,
                        _ => continue,
                    };
                    let rule_content = rule_string.1.trim().to_string();
                    rules.push(Rule::new(rule_type, rule_content));
                }

                Ok(rules)
            }
            RuleSrcType::Geosite(_geosite_update_url, _geosite_category) => {
                todo!("Geosite is not implemented yet");
            }
            RuleSrcType::Unknown => Err("Unknown RuleSrcType".to_string()),
        }
    }
}

impl RuleSrc {
    pub fn new(src_type: RuleSrcType, auto_update: bool) -> Self {
        RuleSrc {
            src_type,
            auto_update,
        }
    }
    pub fn from_file(file_path: String, auto_update: bool) -> Self {
        RuleSrc {
            src_type: RuleSrcType::File(file_path),
            auto_update,
        }
    }
    pub fn from_adguard_home_rule(url: String, auto_update: bool) -> Self {
        RuleSrc {
            src_type: RuleSrcType::AdguardHomeRule(url),
            auto_update,
        }
    }
    pub fn from_geosite(
        geosite_update_url: String,
        geosite_category: String,
        auto_update: bool,
    ) -> Self {
        RuleSrc {
            src_type: RuleSrcType::Geosite(geosite_update_url, geosite_category),
            auto_update,
        }
    }
}

impl Default for RuleSrcType {
    fn default() -> Self {
        RuleSrcType::Unknown
    }
}

impl Config {
    pub fn new(rule_src: Vec<RuleSrc>) -> Self {
        Config { rule_src }
    }
    pub fn add(&mut self, rule_src: RuleSrc) {
        self.rule_src.push(rule_src);
    }
}
