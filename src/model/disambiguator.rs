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
        for (name,card) in &atomics.data {
            let true_name = name.clone();
            let i = name_lookup.insert(true_name.clone(), true_name.clone());
            if i.is_some() {
                panic!("Multiple cards with the same name found!! {}", true_name)
            }
            for n in card.get_names() {
                let i = name_lookup.insert(n, true_name.clone());
                if i.is_some() {
                    panic!("Multiple cards with the same name found!! {}", true_name)
                }
            }
        }
        CardNameDisambiguator::new(name_lookup)
    }
    
    pub fn get(&self, name: &str) -> Option<&String> {
        self.name_lookup.get(name)
    }
}
