use super::rule::Rule;
use std::future::Future;

pub trait Update {
    fn get(&self) -> impl Future<Output = Result<Vec<Rule>, String>>;
}
