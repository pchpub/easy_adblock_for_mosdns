use std::{cell::RefCell, collections::HashMap, fmt::Debug, rc::Rc};

use crate::libs::rule::RuleType;

use super::rule::Rule;

// Merge and remove duplicates
pub fn merge_and_remove_duplicates(rules_vec: Vec<Vec<Rule>>) -> Vec<Rule> {
    let rules_tree = Rc::new(RefCell::new(Domain {
        domain: "".to_string(),
        domain_type: false,
        is_rule: false,
        children: HashMap::new(),
    }));

    for rules in rules_vec {
        for rule in rules {
            if rule.rule_type == RuleType::Keyword || rule.rule_type == RuleType::Regex {
                eprintln!("Keyword and Regex are not supported, skipping");
                continue;
            }
            let mut rule_domain = rule.rule_content.split('.').collect::<Vec<&str>>();
            rule_domain.reverse();
            let rule_type = rule.rule_type;
            let mut current_domain = rules_tree.clone();
            for domain in rule_domain {
                let mut found = false;
                let mut current_domain_temp = current_domain.clone();
                // for child in &mut current_domain.borrow_mut().children {
                //     if child.borrow_mut().domain == domain {
                //         current_domain_temp = child.clone();
                //         found = true;
                //         break;
                //     }
                // }
                if current_domain.borrow().children.contains_key(domain) {
                    current_domain_temp = current_domain.borrow().children[domain].clone();
                    found = true;
                }
                current_domain = current_domain_temp.clone();
                if !found {
                    let new_domain = Domain {
                        domain: domain.to_string(),
                        domain_type: false,
                        is_rule: false,
                        children: HashMap::new(),
                    };
                    // current_domain
                    //     .borrow_mut()
                    //     .children
                    //     .push(Rc::new(RefCell::new(new_domain)));
                    current_domain
                        .borrow_mut()
                        .children
                        .insert(domain.to_string(), Rc::new(RefCell::new(new_domain)));
                    // current_domain_temp =
                    //     current_domain.borrow_mut().children.last().unwrap().clone();
                    current_domain_temp = current_domain.borrow().children[domain].clone();
                    current_domain = current_domain_temp.clone();
                }
            }
            current_domain.borrow_mut().domain_type = match rule_type {
                RuleType::Domain => true,
                RuleType::Full => false,
                RuleType::Keyword => panic!("Keyword is not supported"),
                RuleType::Regex => panic!("Regex is not supported"),
            };
            current_domain.borrow_mut().is_rule = true;
            if current_domain.borrow_mut().domain_type {
                current_domain.borrow_mut().children = HashMap::new();
            }
        }
    }
    let mut rules = vec![];

    // println!("{:#?}", rules_tree.borrow().children);

    // convert rules_tree to rules
    let mut stack: Vec<(Rc<RefCell<Domain>>, usize)> = vec![]; // (domain, deepth)
    stack.push((rules_tree.clone(), 0));
    let mut full_domain: Vec<String> = vec![];
    while !stack.is_empty() {
        let current_domain = stack.pop().unwrap();
        while full_domain.len() > current_domain.1 {
            full_domain.pop();
        }
        full_domain.push(current_domain.0.borrow().domain.clone());
        if current_domain.0.borrow().is_rule {
            let mut full_domain_t = full_domain.clone();
            full_domain_t.reverse();
            full_domain_t.pop();
            rules.push(Rule {
                rule_type: if current_domain.0.borrow().domain_type {
                    RuleType::Domain
                } else {
                    RuleType::Full
                },
                rule_content: full_domain_t.join("."),
            });
        }
        if current_domain.0.borrow().children.len() != 0 {
            stack.extend(
                current_domain
                    .0
                    .borrow()
                    .children
                    .iter()
                    .map(|(_k, v)| (v.clone(), current_domain.1 + 1)),
            );
        }
    }

    rules
}

#[derive(Debug)]
struct Domain {
    domain: String,
    domain_type: bool, // true: 后续子节点都匹配 ,false: 后续子节点不匹配
    is_rule: bool,     // true: 该节点是规则节点, false: 该节点不是规则节点(是中间节点)
    children: HashMap<String, Rc<RefCell<Domain>>>,
}
