use std::io::Write;

use easy_adblock_for_mosdns::libs::{
    config::{Config, RuleSrc, RuleSrcType},
    download::Update,
    tools::merge_and_remove_duplicates,
};
use lazy_static::lazy_static;

lazy_static!(
    static ref HTTP_UA: String = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3".to_string();
);

#[tokio::main]
async fn main() {
    let config = Config {
        rule_src: vec![RuleSrc {
            src_type: RuleSrcType::File("test.txt".to_string()),
            auto_update: false,
        }],
    };
    println!("Getting rules");
    let rules = config.rule_src[0].src_type.get().await.unwrap();
    println!("Got rules");
    let rules_vec = vec![rules];
    println!("Merging rules");
    let rules = merge_and_remove_duplicates(rules_vec);
    // save rules
    let mut file = std::fs::File::create("rules.txt").unwrap();

    for rule in rules {
        // println!("{}", rule);
        file.write_all(format!("{}\n", rule).as_bytes()).unwrap();
    }
    file.flush().unwrap();
}
