use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::card::AtomicCard;
use super::meta::Meta;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Atomics {
    pub data: HashMap<String, AtomicCard>,
    meta: Meta,
}

impl Atomics {
    pub fn get(&self, name: &str) -> Option<AtomicCard> {
        match self.data.get(name) {
            Some(c) => Some(c.clone()),
            None => None,
        }
    }
}
