use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use super::card::Card;
use super::meta::Meta;

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct Atomics {
    // #[serde_as(as = "HashMap<String, CardFace>")]
    data: HashMap<String, Card>,
    meta: Meta,
}

impl Atomics {
    pub fn get( &self, name: &str ) -> Option<&Card> {
        match self.data.get( name ) {
            Some(c) => Some(&c),
            None => None
        }
    }
}

