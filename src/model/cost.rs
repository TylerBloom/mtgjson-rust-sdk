use std::fmt;

use super::symbols::AsManaSymbol;
pub use super::symbols::ManaSymbol;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Hash, Serialize, Deserialize)]
pub struct ManaCost {
    pub cost: Vec<ManaSymbol>,
}

impl fmt::Display for ManaCost {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut digest = String::with_capacity(5 * self.cost.len());
        for m in &self.cost {
            digest += &m.to_string();
        }
        write!(f, "{}", digest)
    }
}

impl From<String> for ManaCost {
    fn from(s: String) -> Self {
        let cost = Vec::new();
        ManaCost { cost }
    }
}

impl ManaCost {
    #[inline]
    fn to_annoted_string(&self) -> String {
        format!("{{{}}}", self.to_string())
    }

    fn mana_value(&self) -> u8 {
        self.cost.iter().map(|s| s.mana_value()).sum()
    }
}
