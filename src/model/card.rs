mod super::card_attribute::CardAttribute;

pub struct Card {
    // Minimal Card Data
    name: String,
    text: String,
    cost: Vec<String>,
    types: Vec<String>,
    
    // Appearance Data
    foil_type: String, // Should be enum
    set: String,
    style: String,
    collector_number: u32,
    
    // Variable card Data
    attributes: Vec<Box<dyn CardAttribute>>,
}
