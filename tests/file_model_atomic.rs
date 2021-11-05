
#[cfg(test)]
mod tests {
    use std::fs;
    use std::env;
    use serde_json;

    use mtgjson::model::atomics::Atomics;
    
    fn get_data_file( name: &str ) -> String {
        let mut digest = env::var("CARGO_MANIFEST_DIR").expect("Project home dir not set.");
        digest += &format!( "/tests/data/{}", name );
        digest
    }

    #[test]
    fn load_single_card() {
        let data_path = get_data_file( "SingleCard.json" );
        println!( "{}", data_path );
        let file_data = fs::read_to_string( data_path ).expect("Data not found.");
        let single_card: Atomics = serde_json::from_str( &file_data ).expect("Data could not be parsed.");
        println!( "{}", serde_json::to_string(single_card.get("Izzet Charm").unwrap()).unwrap() );
    }

    #[test]
    fn reload_single_card() {
        let data_path = get_data_file( "SingleCard.json" );
        println!( "{}", data_path );
        let file_data = fs::read_to_string( data_path ).expect("Data not found.");
        let single_card: Atomics = serde_json::from_str( &file_data ).expect("Data could not be parsed.");
        let single_card_again: Atomics = serde_json::from_str( &serde_json::to_string(&single_card).unwrap() ).expect("Data could not be parsed.");
    }
    
    #[test]
    fn loads_all_cards() {
        let data_path = get_data_file( "PrettyCards.json" );
        println!( "{}", data_path );
        let file_data = fs::read_to_string( data_path ).expect("Data not found.");
        let all_atomics: Atomics = serde_json::from_str( &file_data ).expect("Data could not be parsed.");
        println!( "{}", all_atomics.get("Izzet Charm").unwrap().to_string() );
    }
}
