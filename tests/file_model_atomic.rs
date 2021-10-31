
#[cfg(test)]
mod tests {
    use std::fs;
    use std::env;
    use serde_json;

    use mtgjson::model::atomics::Atomics;

    #[test]
    fn loads_correctly() {
        let mut data_path = env::var("CARGO_MANIFEST_DIR").expect("Project home dir not set.");
        data_path += "/tests/data/AtomicCards.json";
        println!( "{}", data_path );
        let file_data = fs::read_to_string( data_path ).expect("Data not found.");
        let all_atomics: Atomics = serde_json::from_str( &file_data ).expect("Data could not be parsed.");
        println!( "{}", all_atomics.get("Izzet Charm").unwrap().to_string() );
    }
}
