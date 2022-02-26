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
}

#[derive(Clone, Copy, Debug, PartialEq, Hash)]
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

#[derive(Clone, Copy, Debug, PartialEq, Hash)]
pub struct HybridManaSymbol {
    left: BaseManaSymbol,
    right: BaseManaSymbol,
}

#[derive(Clone, Copy, Debug, PartialEq, Hash)]
pub enum BaseOrHybridSymbol {
    Base(BaseManaSymbol),
    Hybrid(HybridManaSymbol),
}

#[derive(Clone, Copy, Debug, PartialEq, Hash)]
pub struct PhyrexianManaSymbol {
    symbol: BaseOrHybridSymbol,
}

#[derive(Clone, Copy, Debug, PartialEq, Hash)]
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
            BaseOrHybridSymbol::Hybrid(s) => s.mana_value()
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
}
