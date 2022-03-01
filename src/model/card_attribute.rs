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

impl<F: AsRef<AtomicCardFace>> From<F> for CardFace {
    fn from(face: F) -> Self {
        let ref_face = face.as_ref();
        let name: String = ref_face.name;
        let cost: ManaCost = ManaCost::from(match ref_face.manaCost {
            Some(c) => c.as_str(),
            None => "",
        });
        /*
        supertypes: HashSet<String>,
        types: HashSet<String>,
        subtypes: HashSet<String>,
        text: String,
        attribs: AttributeMap,
        */
        todo!()
    }
}

impl AttributeMap {
    pub fn insert(
        &mut self,
        attrib_type: AttributeType,
        instance: AttributeInstance,
    ) -> Option<AttributeInstance> {
        self.attribs.insert(attrib_type, instance)
    }
}
