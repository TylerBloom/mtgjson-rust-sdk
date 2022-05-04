use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};

use std::cmp::max;
use std::fmt;

pub trait AsManaSymbol
where
    Self: fmt::Display,
{
    #[inline]
    fn to_annotated_string(&self) -> String {
        format!("{{{}}}", self)
    }

    fn mana_value(&self) -> u8;
    fn from_str(s: &str) -> Option<Self>
    where
        Self: Sized;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum BaseManaSymbol {
    White,
    Blue,
    Black,
    Red,
    Green,
    Colorless,
    Generic(u8),
    Variable,
    Snow,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HybridManaSymbol {
    left: BaseManaSymbol,
    right: BaseManaSymbol,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BaseOrHybridSymbol {
    Base(BaseManaSymbol),
    Hybrid(HybridManaSymbol),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PhyrexianManaSymbol {
    symbol: BaseOrHybridSymbol,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ManaSymbol {
    BaseSymbol(BaseManaSymbol),
    HybridSymbol(HybridManaSymbol),
    PhyrexianSymbol(PhyrexianManaSymbol),
}

impl fmt::Display for BaseManaSymbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                BaseManaSymbol::White => String::from("W"),
                BaseManaSymbol::Blue => String::from("U"),
                BaseManaSymbol::Black => String::from("B"),
                BaseManaSymbol::Red => String::from("R"),
                BaseManaSymbol::Green => String::from("G"),
                BaseManaSymbol::Colorless => String::from("C"),
                BaseManaSymbol::Generic(val) => val.to_string(),
                BaseManaSymbol::Variable => String::from("X"),
                BaseManaSymbol::Snow => String::from("S"),
                _ => String::from("_"),
            }
        )
    }
}

impl AsManaSymbol for BaseManaSymbol {
    #[inline]
    fn mana_value(&self) -> u8 {
        match self {
            BaseManaSymbol::Generic(val) => *val,
            BaseManaSymbol::Variable => 0,
            _ => 1,
        }
    }

    fn from_str(c: &str) -> Option<Self> {
        let color = c.to_lowercase();
        match color.as_str() {
            "w" => Some(BaseManaSymbol::White),
            "u" => Some(BaseManaSymbol::Blue),
            "b" => Some(BaseManaSymbol::Black),
            "r" => Some(BaseManaSymbol::Red),
            "g" => Some(BaseManaSymbol::Green),
            "c" => Some(BaseManaSymbol::Colorless),
            "x" => Some(BaseManaSymbol::Variable),
            "s" => Some(BaseManaSymbol::Snow),
            _ => match color.parse::<u8>() {
                Ok(num) => Some(BaseManaSymbol::Generic(num)),
                _ => None,
            },
        }
    }
}

impl fmt::Display for HybridManaSymbol {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.left, self.right)
    }
}

impl AsManaSymbol for HybridManaSymbol {
    #[inline]
    fn mana_value(&self) -> u8 {
        max(self.left.mana_value(), self.right.mana_value())
    }

    #[inline]
    fn from_str(s: &str) -> Option<Self> {
        lazy_static! {
            static ref HYBRID_RE: Regex = Regex::new(r"(\w+)/(\w+)").unwrap();
        }
        match HYBRID_RE.captures(s) {
            Some(caps) => {
                if caps.len() == 2 {
                    let l = BaseManaSymbol::from_str(caps.get(1).unwrap().as_str());
                    let r = BaseManaSymbol::from_str(caps.get(2).unwrap().as_str());
                    match (l, r) {
                        (Some(left), Some(right)) => Some(HybridManaSymbol { left, right }),
                        _ => None,
                    }
                } else {
                    None
                }
            }
            None => None,
        }
    }
}

impl fmt::Display for BaseOrHybridSymbol {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl AsManaSymbol for BaseOrHybridSymbol {
    #[inline]
    fn mana_value(&self) -> u8 {
        match self {
            BaseOrHybridSymbol::Base(s) => s.mana_value(),
            BaseOrHybridSymbol::Hybrid(s) => s.mana_value(),
        }
    }

    #[inline]
    fn from_str(s: &str) -> Option<Self> {
        if let Some(symbol) = BaseManaSymbol::from_str(s) {
            Some(BaseOrHybridSymbol::Base(symbol))
        } else if let Some(symbol) = HybridManaSymbol::from_str(s) {
            Some(BaseOrHybridSymbol::Hybrid(symbol))
        } else {
            None
        }
    }
}

impl fmt::Display for PhyrexianManaSymbol {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/P", self.symbol)
    }
}

impl AsManaSymbol for PhyrexianManaSymbol {
    #[inline]
    fn mana_value(&self) -> u8 {
        self.symbol.mana_value()
    }

    #[inline]
    fn from_str(s: &str) -> Option<Self> {
        lazy_static! {
            static ref PHYREXIAN_RE: Regex = Regex::new(r"([^/]+)/P").unwrap();
        }
        match PHYREXIAN_RE.find(s) {
            Some(text) => match BaseOrHybridSymbol::from_str(text.as_str()) {
                Some(symbol) => Some(PhyrexianManaSymbol { symbol }),
                None => None,
            },
            None => None,
        }
    }
}

impl fmt::Display for ManaSymbol {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl AsManaSymbol for ManaSymbol {
    #[inline]
    fn mana_value(&self) -> u8 {
        match self {
            ManaSymbol::BaseSymbol(s) => s.mana_value(),
            ManaSymbol::HybridSymbol(s) => s.mana_value(),
            ManaSymbol::PhyrexianSymbol(s) => s.mana_value(),
        }
    }

    #[inline]
    fn from_str(s: &str) -> Option<Self> {
        if let Some(m) = BaseManaSymbol::from_str(s) {
            Some(ManaSymbol::BaseSymbol(m))
        } else if let Some(m) = HybridManaSymbol::from_str(s) {
            Some(ManaSymbol::HybridSymbol(m))
        } else if let Some(m) = PhyrexianManaSymbol::from_str(s) {
            Some(ManaSymbol::PhyrexianSymbol(m))
        } else {
            None
        }
    }
}

impl From<&str> for ManaSymbol {
    fn from(s: &str) -> Self {
        todo!()
    }
}
