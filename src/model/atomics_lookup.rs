use crate::mtgjson::{card::AtomicCard,atomics::Atomics};

use super::disambiguator::CardNameDisambiguator;

pub struct AtomicCardLookup {
    disamb: CardNameDisambiguator,
    atomics: Atomics,
}

impl From<Atomics> for AtomicCardLookup {
    fn from(atomics: Atomics) -> Self {
        let disamb = CardNameDisambiguator::from_atomics(&atomics);
        AtomicCardLookup { disamb, atomics }
    }
}

impl AtomicCardLookup {
    pub fn new(atomics: Atomics, disamb: CardNameDisambiguator) -> Self {
        AtomicCardLookup { atomics, disamb }
    }

    pub fn get(&self, name: &str) -> Option<AtomicCard> {
        if let Some(n) = self.disamb.get(name) {
            self.atomics.get(n)
        } else {
            None
        }
    }
}
