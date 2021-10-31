use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use super::card::Card;
use super::meta::Meta;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Atomics {
    data: HashMap<String, Card>,
    meta: Meta,
}

impl Atomics {
    pub fn get( &self, name: &str ) -> Option<&Card> {
        self.data.get( name )
    }
}

