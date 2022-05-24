use super::colors::Colors;
use super::cost::ManaCost;
use crate::mtgjson::card::AtomicCardFace;

use serde::{Deserialize, Serialize};

use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum AttributeType {
    ColorIndicator,
    Loyalty,
    PowerToughness,
    BottomHalf,
    LifeModifier,
    HandModifier,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AttributeInstance {
    ColorIndicator(ColorIndicator),
    Loyalty(Loyalty),
    PowerToughness(PowerToughness),
    BottomHalf(BottomHalf),
    LifeModifier(LifeModifier),
    HandModifier(HandModifier),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttributeMap {
    pub attribs: HashMap<AttributeType, AttributeInstance>,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct CardFace {
    pub name: String,
    pub cost: ManaCost,
    pub supertypes: HashSet<String>,
    pub types: HashSet<String>,
    pub subtypes: HashSet<String>,
    pub text: String,
    pub attribs: AttributeMap,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ColorIndicator {
    pub colors: HashSet<Colors>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Loyalty {
    Known(i8),
    CDA(String),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Power {
    Known(i8),
    CDA(String),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Toughness {
    Known(i8),
    CDA(String),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PowerToughness {
    pub power: Power,
    pub toughness: Toughness,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BottomHalf {
    pub face: CardFace,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LifeModifier {
    pub modifier: i8,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HandModifier {
    pub modifier: i8,
}

impl CardFace {
    pub fn add_bottom_half(&mut self, face: CardFace) {
        self.attribs.insert(
            AttributeType::BottomHalf,
            AttributeInstance::BottomHalf(BottomHalf { face }),
        );
    }
}

impl From<&AtomicCardFace> for CardFace {
    fn from(face: &AtomicCardFace) -> Self {
        let name: String = face.name.clone();
        let cost: ManaCost = ManaCost::from(face.manaCost.clone().unwrap_or(String::new()));
        let supertypes: HashSet<String> = face.supertypes.iter().map(|s| s.clone()).collect();
        let types: HashSet<String> = face.types.iter().map(|s| s.clone()).collect();
        let subtypes: HashSet<String> = face.subtypes.iter().map(|s| s.clone()).collect();
        let text: String = face.text.clone().unwrap_or(String::new());
        let attribs: AttributeMap = AttributeMap::from(face);
        CardFace {
            name,
            cost,
            supertypes,
            types,
            subtypes,
            text,
            attribs,
        }
    }
}

impl Hash for CardFace {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        let _ = &self.name.hash(state);
    }
}

impl AttributeMap {
    pub fn new() -> Self {
        AttributeMap {
            attribs: HashMap::new(),
        }
    }

    pub fn insert(
        &mut self,
        attrib_type: AttributeType,
        instance: AttributeInstance,
    ) -> Option<AttributeInstance> {
        self.attribs.insert(attrib_type, instance)
    }
}

impl From<&AtomicCardFace> for AttributeMap {
    fn from(face: &AtomicCardFace) -> Self {
        let mut digest = Self::new();
        if let Some(c) = &face.colorIndicator {
            let indicator = ColorIndicator {
                colors: c.iter().filter_map(|i| Colors::from_str(i)).collect(),
            };
            digest.insert(
                AttributeType::ColorIndicator,
                AttributeInstance::ColorIndicator(indicator),
            );
        }
        if let Some(l) = &face.loyalty {
            let loyalty = Loyalty::from(l.as_str());
            digest.insert(AttributeType::Loyalty, AttributeInstance::Loyalty(loyalty));
        }
        if let Some(p) = &face.power {
            let power = Power::from(p.as_str());
            if let Some(t) = &face.toughness {
                let toughness = Toughness::from(t.as_str());
                let pt = PowerToughness { power, toughness };
                digest.insert(
                    AttributeType::PowerToughness,
                    AttributeInstance::PowerToughness(pt),
                );
            }
        }
        if let Some(l) = &face.life {
            let life = LifeModifier {
                modifier: l.parse::<i8>().unwrap(),
            };
            digest.insert(
                AttributeType::LifeModifier,
                AttributeInstance::LifeModifier(life),
            );
        }
        if let Some(h) = &face.hand {
            let hand = HandModifier {
                modifier: h.parse::<i8>().unwrap(),
            };
            digest.insert(
                AttributeType::HandModifier,
                AttributeInstance::HandModifier(hand),
            );
        }
        digest
    }
}

impl From<&str> for Loyalty {
    fn from(l: &str) -> Self {
        if let Ok(loyalty) = l.parse::<i8>() {
            Loyalty::Known(loyalty)
        } else {
            Loyalty::CDA(l.to_string())
        }
    }
}

impl From<&str> for Power {
    fn from(p: &str) -> Self {
        if let Ok(power) = p.parse::<i8>() {
            Power::Known(power)
        } else {
            Power::CDA(p.to_string())
        }
    }
}

impl From<&str> for Toughness {
    fn from(t: &str) -> Self {
        if let Ok(toughness) = t.parse::<i8>() {
            Toughness::Known(toughness)
        } else {
            Toughness::CDA(t.to_string())
        }
    }
}
