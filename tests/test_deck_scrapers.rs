#[cfg(test)]
mod tests {
    use hashbag::HashBag;
    use mtgjson::model::{abstract_card::AbstractCard, atomics_collection::AtomicCardCollection, deck::Deck};
    use serde_json;
    use std::collections::HashMap;
    use std::env;
    use std::fs;
    use serde::{Serialize, Deserialize};

    use mtgjson::mtgjson::atomics::Atomics;

    #[derive(Serialize,Deserialize)]
    struct MockDeck {
        pub name: String,
        pub cards: HashBag<String>
    }

    fn get_data_file(name: &str) -> String {
        let mut digest = env::var("CARGO_MANIFEST_DIR").expect("Project home dir not set.");
        digest += &format!("/tests/data/{}", name);
        digest
    }
    
    fn transform_mock_deck(lookup: &AtomicCardCollection, deck: &MockDeck) -> Deck {
        let mut digest = Deck::new();
        digest.add_name(deck.name.clone());
        for (card, count) in deck.cards.set_iter() {
            println!("{}", card);
            digest.add_card(count, AbstractCard::from(&lookup.get(card).unwrap()));
        }
        digest
    }
    
    fn decks_match(deck_one: &Deck, deck_two: &Deck) -> bool {
        let mut digest = true;
        for (card, count) in deck_one.get_mainboard().set_iter() {
            digest &= deck_two.get_card_count(card) == count;
            if !digest {
                return digest;
            }
        }
        println!( "Passes one way" );
        for (card, count) in deck_two.get_mainboard().set_iter() {
            digest &= deck_one.get_card_count(card) == count;
            if !digest {
                return digest;
            }
        }
        digest
    }

    #[tokio::test]
    async fn test_moxfield_decks() {
        let atomics_path = get_data_file("AtomicCards.json");
        let atomics_data = fs::read_to_string(atomics_path).expect("Data not found.");
        let all_cards: Atomics =
            serde_json::from_str(&atomics_data).expect("Data could not be parsed.");
        let collection = AtomicCardCollection::from(all_cards);
        let data_path = get_data_file("marchesa_decklists.json");
        let file_data = fs::read_to_string(data_path).expect("Data not found.");
        let decks: HashMap<String,MockDeck> = serde_json::from_str(&file_data).expect("Decks could not be parsed.");
        for (url,m_deck) in decks {
            println!("Fetching deck: {}", url);
            let known_deck = transform_mock_deck(&collection, &m_deck);
            let imported_deck = collection.import_deck(url).await;
            assert!(imported_deck.is_ok());
            let unwrapped_deck = imported_deck.unwrap();
            assert!(decks_match(&known_deck, &unwrapped_deck));
            let json = serde_json::to_string(&unwrapped_deck).unwrap();
            let deser_deck: Deck = serde_json::from_str(&json).unwrap();
            assert!(decks_match(&known_deck, &deser_deck));
            assert_eq!(unwrapped_deck, deser_deck);
        }
    }
}
