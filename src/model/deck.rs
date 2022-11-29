use std::hash::Hash;

use hashbag::HashBag;
use serde::{Serialize, Deserialize};

use super::abstract_card::AbstractCard;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Deck {
    pub name: Option<String>,
    pub mainboard: HashBag<AbstractCard>,
    pub sideboard: HashBag<AbstractCard>,
    pub commanders: HashBag<AbstractCard>,
}

impl Deck {
    pub fn new() -> Self {
        Deck {
            name: None,
            mainboard: HashBag::new(),
            sideboard: HashBag::new(),
            commanders: HashBag::new(),
        }
    }

    pub fn add_commander(&mut self, commander: AbstractCard) -> usize {
        self.commanders.insert(commander)
    }

    pub fn remove_commander(&mut self, commander: &AbstractCard) -> usize {
        self.commanders.remove(&commander)
    }

    pub fn add_name(&mut self, name: String) -> Option<String> {
        self.name.replace(name)
    }

    pub fn remove_name(&mut self) -> Option<String> {
        self.name.take()
    }
    
    pub fn get_mainboard(&self) -> &HashBag<AbstractCard> {
        &self.mainboard
    }
    
    pub fn get_sideboard(&self) -> &HashBag<AbstractCard> {
        &self.mainboard
    }
    
    pub fn get_commanders(&self) -> &HashBag<AbstractCard> {
        &self.mainboard
    }

    pub fn get_card_count(&self, card: &AbstractCard) -> usize {
        self.mainboard.get(card).map_or(0usize, |(_, count)| count)
            + self.sideboard.get(card).map_or(0usize, |(_, count)| count)
            + self.commanders.get(card).map_or(0usize, |(_, count)| count)
    }

    pub fn add_card(&mut self, count: usize, card: AbstractCard) -> usize {
        self.mainboard.insert_many(card, count)
    }

    pub fn remove_card(&mut self, mut count: usize, card: &AbstractCard) -> usize {
        let mut digest: usize = 0;
        while count != 0 && self.mainboard.remove(card) != 0 {
            digest += 1;
            count -= 1;
        }
        digest
    }

    pub fn add_sideboard_card(&mut self, count: usize, card: AbstractCard) -> usize {
        self.sideboard.insert_many(card, count)
    }

    pub fn remove_sideboard_card(&mut self, mut count: usize, card: &AbstractCard) -> usize {
        let mut digest: usize = 0;
        while count != 0 && self.sideboard.remove(card) != 0 {
            digest += 1;
            count -= 1;
        }
        digest
    }
}

impl Hash for Deck {
    // TODO: This needs to be improved...
    // Perhaps we can order the cards?
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}
