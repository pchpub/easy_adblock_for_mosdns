use std::io::Write;

use easy_adblock_for_mosdns::libs::{
    config::Config, download::Update, tools::merge_and_remove_duplicates,
};
use lazy_static::lazy_static;

lazy_static!(
    static ref HTTP_UA: String = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3".to_string();
);

#[tokio::main]
async fn main() {
    let config = match Config::load("config.json") {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Failed to load config: {}", e);
            return;
        }
    };

    for accept_rule in [true, false] {
        let mut file = std::fs::File::create(if accept_rule {
            &config.accept_rule_path
        } else {
            &config.reject_rule_path
        })
        .unwrap();
        let mut rules_vec = vec![];
        for rule_src in &config.rule_src {
            let rules = rule_src.src_type.get(accept_rule).await.unwrap();
            rules_vec.push(rules);
        }
        let rules = merge_and_remove_duplicates(rules_vec);
        for rule in rules {
            file.write_all(format!("{}\n", rule).as_bytes()).unwrap();
        }
    }
}
