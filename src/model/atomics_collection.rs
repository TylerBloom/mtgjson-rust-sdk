use std::error::Error;
use std::fmt;

use cycle_map::{CycleMap, GroupMap};
use hyper::{body, client::connect::HttpConnector, Client, Request};
use hyper_tls::HttpsConnector;
use lazy_static::lazy_static;
use regex::Regex;

use super::{
    abstract_card::AbstractCard, deck::Deck, deck_sites::moxfield::MoxfieldDeck,
};
use crate::{
    mtgjson::{atomics::Atomics, card::AtomicCard},
    utils::response_into_string,
};

pub struct AtomicCardCollection {
    cards: GroupMap<String, AtomicCard>,
}

impl From<Atomics> for AtomicCardCollection {
    #[inline]
    fn from(atomics: Atomics) -> Self {
        Self::new(atomics)
    }
}

impl AtomicCardCollection {
    pub fn new(atomics: Atomics) -> Self {
        let mut cards = GroupMap::with_capacity(atomics.data.len());
        for (_, c) in &atomics.data {
            cards.insert_right(c.clone());
            for n in c.get_names() {
                cards.insert_left(n, &c);
            }
        }
        Self { cards }
    }

    pub fn create_deck(&self, raw_deck: String) -> Option<Deck> {
        /*
         * This will be the single deck-creation interface for strings.
         * If a special deck format needs to be supported, the `get` method can help.
         */
        lazy_static! {
            static ref DECK_RE: Regex =
                Regex::new(r"^([0-9]+)x?[\s^\n]+([a-zA-Z\,\ /-]+)$").unwrap();
        }
        todo!()
    }

    pub async fn import_deck(&self, url: String) -> Option<Deck> {
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
            Ok(deck)
        } else if let Some(caps) = GOLDFISH_URL_RE.captures(&url) {
            todo!();
            let mut deck = Deck::new();
            Ok(deck)
        } else if let Some(caps) = MOXFIELD_URL_RE.captures(&url) {
            let raw_deck: MoxfieldDeck = self
                .get_moxfield_deck(
                    &client,
                    format!("https://api.moxfield.com/v2/decks/all/{}", &caps[1]),
                )
                .await.ok()?;
            let mut deck = Deck::new();
            for (name, card) in raw_deck.mainboard {
                if let Some(c) = self.get(&name) {
                    deck.add_card(card.quantity, AbstractCard::from(&c));
                } else {
                    return None;
                }
            }
            for (name, card) in raw_deck.sideboard {
                if let Some(c) = self.get(&name) {
                    deck.add_sideboard_card(card.quantity, AbstractCard::from(&c));
                } else {
                    return None;
                }
            }
            for (name, card) in raw_deck.commanders {
                if let Some(c) = self.get(&name) {
                    deck.add_commander(AbstractCard::from(&c));
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

    pub fn get(&self, name: &String) -> Option<AtomicCard> {
        self.cards.get_right(name).cloned()
    }

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

#[derive(Debug)]
pub struct AtomicsError {}

impl fmt::Display for AtomicsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Something went wrong...")
    }
}

impl Error for AtomicsError {}
