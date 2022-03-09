use std::error::Error;
use std::fmt;

use lazy_static::lazy_static;
use regex::Regex;
use hyper::{body, client::connect::HttpConnector, Client, Request};
use hyper_tls::HttpsConnector;

use crate::{
    utils::response_into_string,
    mtgjson::{card::AtomicCard, atomics::Atomics}
};
use super::{abstract_card::AbstractCard, deck::Deck, deck_sites::moxfield::MoxfieldDeck, disambiguator::CardNameDisambiguator};

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

    pub async fn import_deck(&self, url: String) -> Result<Deck, Box<dyn Error>> {
        /*
         * This will be the single deck-creation interface for scraping decks from the web.
         * I can't support all deck sites, but new ones can be added.
         * Also, library users can scrape the site themselves and use the other interfaces to
         * create a deck if need be.
         */
        lazy_static!{
            static ref SCRYFALL_URL_RE: Regex = Regex::new(r"^$").unwrap();
            static ref GOLDFISH_URL_RE: Regex = Regex::new(r"^$").unwrap();
            static ref MOXFIELD_URL_RE: Regex = Regex::new(r"^https://www.moxfield.com/decks/(\w+)$").unwrap();
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
            let raw_deck: MoxfieldDeck = self.get_moxfield_deck(&client, format!("https://api.moxfield.com/v2/decks/all/{}", &caps[0])).await?;
            let mut deck = Deck::new();
            for (name, card) in raw_deck.mainboard {
                if let Some(c) = self.get(&name) {
                    deck.add_card(card.quantity, AbstractCard::from(&c));
                } else {
                    return Err(Box::new(AtomicsError {}));
                }
            }
            for (name, card) in raw_deck.sideboard {
                if let Some(c) = self.get(&name) {
                    deck.add_sideboard_card(card.quantity, AbstractCard::from(&c));
                } else {
                    return Err(Box::new(AtomicsError {}));
                }
            }
            for (name, card) in raw_deck.commanders {
                if let Some(c) = self.get(&name) {
                    deck.add_commander(AbstractCard::from(&c));
                } else {
                    return Err(Box::new(AtomicsError {}));
                }
            }
            Ok(deck)
        } else if let Some(caps) = TAPPEDOUT_URL_RE.captures(&url) {
            todo!();
            let mut deck = Deck::new();
            Ok(deck)
        } else {
            Err(Box::new(AtomicsError {}))
        }
    }

    pub fn get(&self, name: &str) -> Option<AtomicCard> {
        if let Some(n) = self.disamb.get(name) {
            self.atomics.get(n)
        } else {
            None
        }
    }

    async fn get_moxfield_deck(&self, client: &hyper::Client<HttpsConnector<HttpConnector>, body::Body>, url: String) -> Result<MoxfieldDeck, Box<dyn Error>> {
        let res = client.request(
            Request::builder()
            .method("GET")
            .uri(url)
            .body(body::Body::from(""))
            .unwrap(),
        ).await?;
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
