use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use super::card::AtomicCard;
use super::meta::Meta;

#[derive(Serialize, Deserialize)]
pub struct Atomics {
    data: HashMap<String, AtomicCard>,
    meta: Meta,
}

impl Atomics {
    pub fn get( &self, name: &str ) -> Option<&AtomicCard> {
        match self.data.get( name ) {
            Some(c) => Some(&c),
            None => None
        }
    }
}

