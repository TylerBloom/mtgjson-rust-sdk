use super::symbols::AsManaSymbol;
pub use super::symbols::ManaSymbol;

use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};

use std::fmt;

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
        lazy_static! {
            static ref BRACE_RE: Regex = Regex::new(r"\{([^}]+)\}").unwrap();
        };
        let cost: Vec<ManaSymbol> = BRACE_RE
            .captures_iter(&s)
            .filter_map(|c| ManaSymbol::from_str(c.get(1).unwrap().as_str()))
            .collect();
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
