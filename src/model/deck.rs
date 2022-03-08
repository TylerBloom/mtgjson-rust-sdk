use hashbag::HashBag;

use super::abstract_card::AbstractCard;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Deck {
    name: Option<String>,
    cards: HashBag<AbstractCard>,
}

impl Deck {
    pub fn new(cards: HashBag<AbstractCard>) -> Self {
        Deck { name: None, cards }
    }

    pub fn with_name(name: String, cards: HashBag<AbstractCard>) -> Self {
        Deck {
            name: Some(name),
            cards,
        }
    }

    pub fn get_card_count(&self, card: &AbstractCard) -> usize {
        self.cards.get(card).map_or(0usize, |(_, count)| count)
    }

    pub fn add_card(&mut self, count: usize, card: AbstractCard) -> usize {
        self.cards.insert_many(card, count)
    }

    pub fn remove_card(&mut self, mut count: usize, card: &AbstractCard) -> usize {
        let mut digest: usize = 0;
        while count != 0 && self.cards.remove(card) != 0 {
            digest += 1;
            count -= 1;
        }
        digest
    }
}
