use lazy_static::lazy_static;
use regex::Regex;

use crate::mtgjson::{card::AtomicCard,atomics::Atomics};
use super::{deck::Deck, disambiguator::CardNameDisambiguator};

pub struct AtomicCardCollection {
    disamb: CardNameDisambiguator,
    atomics: Atomics,
}

impl From<Atomics> for AtomicCardCollection {
    fn from(atomics: Atomics) -> Self {
        let disamb = CardNameDisambiguator::from_atomics(&atomics);
        AtomicCardCollection { disamb, atomics }
    }
}

impl AtomicCardCollection {
    pub fn new(atomics: Atomics, disamb: CardNameDisambiguator) -> Self {
        AtomicCardCollection { atomics, disamb }
    }

    pub fn create_deck(&self, raw_deck: String) -> Result<Deck, ()> {
        /*
         * This will be the single deck-creation interface for strings.
         * If a special deck format needs to be supported, the `get` method can help.
         */
        lazy_static!{
            static ref DECK_RE: Regex = Regex::new(r"^([0-9]+)x?[\s^\n]+([a-zA-Z\,\ /-]+)$").unwrap();
        }
        todo!()
    }

    pub async fn import_deck(&self, url: String) -> Result<Deck, ()> {
        /*
         * This will be the single deck-creation interface for scraping decks from the web.
         * I can't support all deck sites, but new ones can be added.
         * Also, library users can scrape the site themselves and use the other interfaces to
         * create a deck if need be.
         */
        lazy_static!{
            static ref SCRYFALL_URL_RE: Regex = todo!();
            static ref GOLDFISH_URL_RE: Regex = todo!();
            static ref MOXFIELD_URL_RE: Regex = todo!();
            static ref TAPPEDOUT_URL_RE: Regex = todo!();
        }
        if SCRYFALL_URL_RE.is_match(&url) {
            let raw_deck: String = todo!();
            self.create_deck(raw_deck)
        } else if GOLDFISH_URL_RE.is_match(&url) {
            let raw_deck: String = todo!();
            self.create_deck(raw_deck)
        } else if MOXFIELD_URL_RE.is_match(&url) {
            let raw_deck: String = todo!();
            self.create_deck(raw_deck)
        } else if TAPPEDOUT_URL_RE.is_match(&url) {
            let raw_deck: String = todo!();
            self.create_deck(raw_deck)
        } else {
            Err(())
        }
    }

    pub fn get(&self, name: &str) -> Option<AtomicCard> {
        if let Some(n) = self.disamb.get(name) {
            self.atomics.get(n)
        } else {
            None
        }
    }
}
