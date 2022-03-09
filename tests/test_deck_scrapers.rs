#[cfg(test)]
mod tests {
    use mtgjson::model::abstract_card::AbstractCard;
    use mtgjson::model::atomics_collection::AtomicCardCollection;
    use serde_json;
    use std::env;
    use std::fs;

    use mtgjson::mtgjson::atomics::Atomics;

    fn get_data_file(name: &str) -> String {
        let mut digest = env::var("CARGO_MANIFEST_DIR").expect("Project home dir not set.");
        digest += &format!("/tests/data/{}", name);
        digest
    }

    #[tokio::test]
    async fn test_moxfield_decks() {
        let decks = vec![
            "https://www.moxfield.com/decks/A0PZaTfza0e7PIG7oYIt0Q",
            "https://www.moxfield.com/decks/E9idN4nGJ0qgiHCuqTCwrQ",
            "https://www.moxfield.com/decks/ML3RrfBEb0KN6bSiG5NU5g",
            "https://www.moxfield.com/decks/x5PAwnIoI0menRuz8XsZ4w",
            "https://www.moxfield.com/decks/AKW6DhTl-UaOylut3-Yl8g",
            "https://www.moxfield.com/decks/ihTvEOsjXEG8-mNesQJcdg",
            "https://www.moxfield.com/decks/q4rZygA2Bk28zuit0_vM9w",
            "https://www.moxfield.com/decks/DpCYADuUp0qJ_SM2I-0m-Q",
            "https://www.moxfield.com/decks/ubWRvOFLNk2ts7MEx2WsSA",
            "https://www.moxfield.com/decks/Wbfq-9ZDV0S6_FlhSvYV_g",
            "https://www.moxfield.com/decks/4SsLum3MUEiHChXzIqX3Ww",
            "https://www.moxfield.com/decks/rxGCse6-Hk6ctO4YNCDNVA",
            "https://www.moxfield.com/decks/raDlX55510iXKQQEKJXMkg",
        ];
        let data_path = get_data_file("AtomicCards.json");
        let file_data = fs::read_to_string(data_path).expect("Data not found.");
        let all_cards: Atomics =
            serde_json::from_str(&file_data).expect("Data could not be parsed.");
        let collection = AtomicCardCollection::from(all_cards);
        for deck in decks {
            println!("Fetching deck: {}", deck);
            assert!(collection.import_deck(deck.to_string()).await.is_ok());
        }
    }
}
