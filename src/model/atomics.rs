use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use super::card::Card;
use super::meta::Meta;

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct Atomics {
    // #[serde_as(as = "HashMap<String, Vec<Card>>")]
    data: HashMap<String, [Card; 1]>,
    meta: Meta,
}

impl Atomics {
    pub fn get( &self, name: &str ) -> Option<&Card> {
        match self.data.get( name ) {
            Some(c) => Some(&c[0]),
            None => None
        }
    }
}

