use serde::{Deserialize, Serialize};

use std::{collections::HashMap, fmt};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct MoxfieldDeck {
    pub mainboard: HashMap<String, MoxfieldCard>,
    pub sideboard: HashMap<String, MoxfieldCard>,
    pub maybeboard: HashMap<String, MoxfieldCard>,
    pub commanders: HashMap<String, MoxfieldCard>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct MoxfieldCard {
    pub quantity: usize,
}

impl fmt::Display for MoxfieldDeck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut digest = String::new();
        digest += "MAINBOARD:\n";
        for (name, card) in &self.mainboard {
            digest += &format!("{} {}\n", card.quantity, name);
        }
        digest += "\nSIDEBOARD:\n";
        for (name, card) in &self.sideboard {
            digest += &format!("{} {}\n", card.quantity, name);
        }
        digest += "\nMAYBEBOARD:\n";
        for (name, card) in &self.maybeboard {
            digest += &format!("{} {}\n", card.quantity, name);
        }
        digest += "\nCOMMANDERS:";
        for (name, card) in &self.commanders {
            digest += &format!("{} {}\n", card.quantity, name);
        }
        write!(f, "{}", digest)?;
        Ok(())
    }
}
