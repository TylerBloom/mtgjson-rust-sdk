use super::colors::Colors;
use super::cost::ManaCost;
use crate::mtgjson::card::AtomicCardFace;

use serde::{Deserialize, Serialize};

use std::collections::{HashMap, HashSet};

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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AttributeInstance {
    ColorIndicator(ColorIndicator),
    Loyalty(Loyalty),
    PowerToughness(PowerToughness),
    BottomHalf(BottomHalf),
    LifeModifier(LifeModifier),
    HandModifier(HandModifier),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AttributeMap {
    attribs: HashMap<AttributeType, AttributeInstance>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CardFace {
    name: String,
    cost: ManaCost,
    supertypes: HashSet<String>,
    types: HashSet<String>,
    subtypes: HashSet<String>,
    text: String,
    attribs: AttributeMap,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ColorIndicator {
    colors: HashSet<Colors>,
}

#[derive(Clone, Debug, PartialEq, Hash, Serialize, Deserialize)]
pub struct Loyalty {
    loyalty: u8,
}

#[derive(Clone, Debug, PartialEq, Hash, Serialize, Deserialize)]
pub enum Power {
    Known(u8),
    CDA(String),
}

#[derive(Clone, Debug, PartialEq, Hash, Serialize, Deserialize)]
pub enum Toughness {
    Known(u8),
    CDA(String),
}

#[derive(Clone, Debug, PartialEq, Hash, Serialize, Deserialize)]
pub struct PowerToughness {
    power: Power,
    toughness: Toughness,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BottomHalf {
    face: CardFace,
}

#[derive(Clone, Debug, PartialEq, Hash, Serialize, Deserialize)]
pub struct LifeModifier {
    modifier: i8,
}

#[derive(Clone, Debug, PartialEq, Hash, Serialize, Deserialize)]
pub struct HandModifier {
    modifier: i8,
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
        /*
            ColorIndicator(ColorIndicator),
            Loyalty(Loyalty),
            PowerToughness(PowerToughness),
            BottomHalf(BottomHalf),
            LifeModifier(LifeModifier),
            HandModifier(HandModifier),
        */
        todo!()
    }
}
