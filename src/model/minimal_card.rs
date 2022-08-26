use std::error::Error;

use serde::{Deserialize, Serialize};

use cycle_map::CycleMap;
use hashbag::HashBag;

#[cfg(feature = "deck_sites")]
use hyper::{body, client::connect::HttpConnector, Client, Request};
#[cfg(feature = "deck_sites")]
use hyper_tls::HttpsConnector;

use regex::Regex;

use crate::model::{atomics_collection::AtomicCardCollection, deck::Deck};
#[cfg(feature = "deck_sites")]
use crate::{model::deck_sites::moxfield::MoxfieldDeck, utils::response_into_string};

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
pub struct MinimalCardFace {
    pub face_name: String,
    pub text: String,
    pub types: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
pub struct MinimalCard {
    pub name: String,
    pub oracle_id: String,
    pub faces: Vec<MinimalCardFace>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct MinimalDeck {
    pub name: Option<String>,
    pub mainboard: HashBag<MinimalCard>,
    pub sideboard: HashBag<MinimalCard>,
    pub commanders: HashBag<MinimalCard>,
}

impl MinimalDeck {
    pub fn new() -> Self {
        MinimalDeck {
            name: None,
            mainboard: HashBag::new(),
            sideboard: HashBag::new(),
            commanders: HashBag::new(),
        }
    }

    pub fn add_commander(&mut self, commander: MinimalCard) -> usize {
        self.commanders.insert(commander)
    }

    pub fn remove_commander(&mut self, commander: &MinimalCard) -> usize {
        self.commanders.remove(&commander)
    }

    pub fn add_name(&mut self, name: String) -> Option<String> {
        self.name.replace(name)
    }

    pub fn remove_name(&mut self) -> Option<String> {
        self.name.take()
    }

    pub fn get_mainboard(&self) -> &HashBag<MinimalCard> {
        &self.mainboard
    }

    pub fn get_sideboard(&self) -> &HashBag<MinimalCard> {
        &self.mainboard
    }

    pub fn get_commanders(&self) -> &HashBag<MinimalCard> {
        &self.mainboard
    }

    pub fn get_card_count(&self, card: &MinimalCard) -> usize {
        self.mainboard.get(card).map_or(0usize, |(_, count)| count)
            + self.sideboard.get(card).map_or(0usize, |(_, count)| count)
            + self.commanders.get(card).map_or(0usize, |(_, count)| count)
    }

    pub fn add_card(&mut self, count: usize, card: MinimalCard) -> usize {
        self.mainboard.insert_many(card, count)
    }

    pub fn remove_card(&mut self, mut count: usize, card: &MinimalCard) -> usize {
        let mut digest: usize = 0;
        while count != 0 && self.mainboard.remove(card) != 0 {
            digest += 1;
            count -= 1;
        }
        digest
    }

    pub fn add_sideboard_card(&mut self, count: usize, card: MinimalCard) -> usize {
        self.sideboard.insert_many(card, count)
    }

    pub fn remove_sideboard_card(&mut self, mut count: usize, card: &MinimalCard) -> usize {
        let mut digest: usize = 0;
        while count != 0 && self.sideboard.remove(card) != 0 {
            digest += 1;
            count -= 1;
        }
        digest
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MinimalCardCollection {
    pub cards: CycleMap<String, MinimalCard>,
}

impl MinimalCardCollection {
    pub fn new(atomics: AtomicCardCollection, lang: &String) -> Self {
        let mut cards = CycleMap::with_capacity(atomics.cards.len_right());
        for c in atomics.cards.iter_right() {
            let mc = c.as_minimal(lang).unwrap();
            let name = mc.name.clone();
            cards.insert(name, mc);
        }
        Self { cards }
    }

    pub fn create_deck(&self, _raw_deck: String) -> Option<MinimalDeck> {
        todo!()
    }

    #[cfg(feature = "deck_sites")]
    pub async fn import_deck(&self, url: String) -> Option<Deck> {
        use lazy_static::lazy_static;

        /*
         * This will be the single deck-creation interface for scraping decks from the web.
         * I can't support all deck sites, but new ones can be added.
         * Also, library users can scrape the site themselves and use the other interfaces to
         * create a deck if need be.
         */
        lazy_static! {
            static ref SCRYFALL_URL_RE: Regex = Regex::new(r"^$").unwrap();
            static ref GOLDFISH_URL_RE: Regex = Regex::new(r"^$").unwrap();
            static ref MOXFIELD_URL_RE: Regex =
                Regex::new(r"^https://www.moxfield.com/decks/([\w-]+)$").unwrap();
            static ref TAPPEDOUT_URL_RE: Regex = Regex::new(r"^$").unwrap();
        }
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        if let Some(caps) = SCRYFALL_URL_RE.captures(&url) {
            todo!();
            let mut deck = Deck::new();
            Some(deck)
        } else if let Some(caps) = GOLDFISH_URL_RE.captures(&url) {
            todo!();
            let mut deck = Deck::new();
            Some(deck)
        } else if let Some(caps) = MOXFIELD_URL_RE.captures(&url) {
            let raw_deck: MoxfieldDeck = self
                .get_moxfield_deck(
                    &client,
                    format!("https://api.moxfield.com/v2/decks/all/{}", &caps[1]),
                )
                .await
                .ok()?;
            let mut deck = Deck::new();
            for (name, card) in raw_deck.mainboard {
                if let Some(c) = self.get(&name) {
                    deck.add_card(card.quantity, MinimalCard::from(c));
                } else {
                    return None;
                }
            }
            for (name, card) in raw_deck.sideboard {
                if let Some(c) = self.get(&name) {
                    deck.add_sideboard_card(card.quantity, MinimalCard::from(&c));
                } else {
                    return None;
                }
            }
            for (name, card) in raw_deck.commanders {
                if let Some(c) = self.get(&name) {
                    deck.add_commander(MinimalCard::from(&c));
                } else {
                    return None;
                }
            }
            Some(deck)
        } else if let Some(caps) = TAPPEDOUT_URL_RE.captures(&url) {
            todo!();
            let mut deck = Deck::new();
            Some(deck)
        } else {
            None
        }
    }

    pub fn get(&self, name: &String) -> Option<MinimalCard> {
        self.cards.get_right(name).cloned()
    }

    #[cfg(feature = "deck_sites")]
    async fn get_moxfield_deck(
        &self,
        client: &hyper::Client<HttpsConnector<HttpConnector>, body::Body>,
        url: String,
    ) -> Result<MoxfieldDeck, Box<dyn Error>> {
        let res = client
            .request(
                Request::builder()
                    .method("GET")
                    .uri(url)
                    .body(body::Body::from(""))
                    .unwrap(),
            )
            .await?;
        Ok(serde_json::from_str(&response_into_string(res).await?)?)
    }
}
