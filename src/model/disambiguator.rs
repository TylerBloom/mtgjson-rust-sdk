use std::collections::HashMap;

use crate::mtgjson::atomics::Atomics;

#[derive(Debug, Clone)]
pub struct CardNameDisambiguator {
    name_lookup: HashMap<String, String>,
}

impl CardNameDisambiguator {
    pub fn new(name_lookup: HashMap<String, String>) -> Self {
        CardNameDisambiguator { name_lookup }
    }

    pub fn from_atomics(atomics: &Atomics) -> Self {
        let mut name_lookup = HashMap::with_capacity(atomics.data.len());
        for (true_name, card) in &atomics.data {
            for n in card.get_names() {
                let i = name_lookup.insert(n.clone(), true_name.clone());
                if i.is_some() {
                    println!("Multiple cards with the same name found!!\n\ttrue name: {}\n\talias: {}\n", true_name, n)
                }
            }
        }
        CardNameDisambiguator::new(name_lookup)
    }

    pub fn get(&self, name: &str) -> Option<&String> {
        self.name_lookup.get(name)
    }
}
