use crate::libs::request::{RequestMethod, RequestStructure};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

lazy_static!(
    static ref HTTP_UA: String = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3".to_string();
);

use super::{
    download::Update,
    rule::{Rule, RuleType},
};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub rule_src: Vec<RuleSrc>,
    pub accept_rule_path: String,
    pub reject_rule_path: String,
}

#[derive(Deserialize, Serialize)]
pub struct RuleSrc {
    pub src_type: RuleSrcType,
    pub auto_update: bool,
}

#[derive(Deserialize, Serialize)]
pub enum RuleSrcType {
    MosdnsFile(String, bool),      // file_path, accept/reject(true/false)
    PureFile(String, bool),        // file_path, accept/reject(true/false)
    AdguardHomeRule(String),       // URL, accept/reject(true/false)
    Geosite(String, String, bool), // geosite_update_url, geosite_category, accept/reject(true/false)
    Unknown,
}

impl Update for RuleSrcType {
    async fn get(&self, want_accept_rule: bool) -> Result<Vec<Rule>, String> {
        match self {
            RuleSrcType::MosdnsFile(file_path, accept_rule) => {
                if want_accept_rule != *accept_rule {
                    return Ok(vec![]);
                }
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
            RuleSrcType::PureFile(path, accept_rule) => {
                if want_accept_rule != *accept_rule {
                    return Ok(vec![]);
                }
                let mut rules = vec![];
                // read file
                let file = std::fs::File::open(path);
                if file.is_err() {
                    return Err("Failed to open file".to_string());
                }
                let file = file.unwrap();
                let mut reader = std::io::BufReader::new(file);
                let mut buf = String::new();
                reader.read_to_string(&mut buf).unwrap();
                for line in buf.lines() {
                    let rule_type = RuleType::Domain;
                    let rule_content = line.trim().to_string();
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
                if want_accept_rule {
                    for line in content.lines() {
                        if line.starts_with("@@||") && line.ends_with("^") {
                            // println!("{}", line);
                            let rule_content =
                                line.trim_start_matches("@@||").trim_end_matches("^");
                            rules.push(Rule::new(RuleType::Domain, rule_content.to_string()));
                        }
                    }
                } else {
                    for line in content.lines() {
                        if line.starts_with("||") && line.ends_with("^") {
                            // println!("{}", line);
                            let rule_content = line.trim_start_matches("||").trim_end_matches("^");
                            rules.push(Rule::new(RuleType::Domain, rule_content.to_string()));
                        }
                    }
                }
                Ok(rules)
            }
            RuleSrcType::Geosite(_geosite_update_url, _geosite_category, _accept_rule) => {
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
    pub fn from_mosdns_file(file_path: String, accept_rule: bool, auto_update: bool) -> Self {
        RuleSrc {
            src_type: RuleSrcType::MosdnsFile(file_path, accept_rule),
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
        accept_rule: bool,
        auto_update: bool,
    ) -> Self {
        RuleSrc {
            src_type: RuleSrcType::Geosite(geosite_update_url, geosite_category, accept_rule),
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
        Config {
            rule_src,
            accept_rule_path: "./accept.txt".to_string(),
            reject_rule_path: "./reject.txt".to_string(),
        }
    }
    pub fn add(&mut self, rule_src: RuleSrc) {
        self.rule_src.push(rule_src);
    }
    pub fn load(file_path: &str) -> Result<Self, String> {
        let file = std::fs::File::open(file_path);
        if file.is_err() {
            return Err("Failed to open file".to_string());
        }
        let file = file.unwrap();
        let reader = std::io::BufReader::new(file);
        let config: Config = serde_json::from_reader(reader).unwrap();
        Ok(config)
    }
    pub fn save(&self, file_path: &str) -> Result<(), String> {
        let file = std::fs::File::create(file_path);
        if file.is_err() {
            return Err("Failed to create file".to_string());
        }
        let mut file = file.unwrap();
        let json_config = serde_json::to_string_pretty(self).unwrap();
        file.write_all(json_config.as_bytes()).unwrap();
        Ok(())
    }
}
