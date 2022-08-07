#[cfg(test)]
mod tests {
    use mtgjson::model::abstract_card::AbstractCard;
    use serde_json;
    use std::env;
    use std::fs;

    use mtgjson::mtgjson::atomics::Atomics;

    fn get_data_file(name: &str) -> String {
        let mut digest = env::var("CARGO_MANIFEST_DIR").expect("Project home dir not set.");
        digest += &format!("/tests/data/{}", name);
        digest
    }

    #[test]
    fn load_all_cards() {
        let data_path = get_data_file("AtomicCards.json");
        println!("{}", data_path);
        let file_data = fs::read_to_string(data_path).expect("Data not found.");
        let all_cards: Atomics =
            serde_json::from_str(&file_data).expect("Data could not be parsed.");
        for (n, c) in all_cards.data {
            println!("Converting {}", n);
            let ab = AbstractCard::from(&c);
            let _s = serde_json::to_string(&ab).expect(&format!("Could not convert {} to string", ab.get_name()));
            let _mc = c.as_minimal(&"English".into()).expect(&format!("Could not minimize {} in English", ab.get_name()));
            let _mc = c.as_minimal(&"Spanish".into()).expect(&format!("Could not minimize {} in Spanish", ab.get_name()));
            let _mc = c.as_minimal(&"Chinese Simplified".into()).expect(&format!("Could not minimize {} in Chinese Simplified", ab.get_name()));
        }
    }

    #[test]
    fn load_single_card() {
        let data_path = get_data_file("SingleSimpleCreature.json");
        println!("{}", data_path);
        let file_data = fs::read_to_string(data_path).expect("Data not found.");
        let single_card: Atomics =
            serde_json::from_str(&file_data).expect("Data could not be parsed.");
        println!(
            "{}",
            serde_json::to_string(&single_card.get("Grizzly Bears").unwrap()).unwrap()
        );
    }

    #[test]
    fn reload_single_card() {
        let data_path = get_data_file("SingleSimpleCreature.json");
        println!("{}", data_path);
        let file_data = fs::read_to_string(data_path).expect("Data not found.");
        let single_card: Atomics =
            serde_json::from_str(&file_data).expect("Data could not be parsed.");
        let _single_card_again: Atomics =
            serde_json::from_str(&serde_json::to_string(&single_card).unwrap())
                .expect("Data could not be parsed.");
    }

    #[test]
    fn load_single_spell() {
        let data_path = get_data_file("SingleSpell.json");
        println!("{}", data_path);
        let file_data = fs::read_to_string(data_path).expect("Data not found.");
        let single_card: Atomics =
            serde_json::from_str(&file_data).expect("Data could not be parsed.");
        println!(
            "{}",
            serde_json::to_string(&single_card.get("Izzet Charm").unwrap()).unwrap()
        );
    }

    #[test]
    fn load_single_creature() {
        let data_path = get_data_file("SingleCreature.json");
        println!("{}", data_path);
        let file_data = fs::read_to_string(data_path).expect("Data not found.");
        let single_card: Atomics =
            serde_json::from_str(&file_data).expect("Data could not be parsed.");
        println!(
            "{}",
            serde_json::to_string(&single_card.get("Torrential Gearhulk").unwrap()).unwrap()
        );
    }

    #[test]
    fn load_single_dfc() {
        let data_path = get_data_file("SingleDFC.json");
        println!("{}", data_path);
        let file_data = fs::read_to_string(data_path).expect("Data not found.");
        let single_card: Atomics =
            serde_json::from_str(&file_data).expect("Data could not be parsed.");
        println!(
            "{}",
            serde_json::to_string(
                &single_card
                    .get("Delver of Secrets // Insectile Aberration")
                    .unwrap()
            )
            .unwrap()
        );
    }

    #[test]
    fn load_single_split() {
        let data_path = get_data_file("SingleSplit.json");
        println!("{}", data_path);
        let file_data = fs::read_to_string(data_path).expect("Data not found.");
        let single_card: Atomics =
            serde_json::from_str(&file_data).expect("Data could not be parsed.");
        println!(
            "{}",
            serde_json::to_string(&single_card.get("Commit // Memory").unwrap()).unwrap()
        );
    }

    /* File is too large to move around
    #[test]
    fn loads_all_cards() {
        let data_path = get_data_file( "AtomicCards.json" );
        println!( "{}", data_path );
        let file_data = fs::read_to_string( data_path ).expect("Data not found.");
        let all_atomics: Atomics = serde_json::from_str( &file_data ).expect("Data could not be parsed.");
        println!( "{}", all_atomics.get("Izzet Charm").unwrap().to_string() );
    }
    */
}
