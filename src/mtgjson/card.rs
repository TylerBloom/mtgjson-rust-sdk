use super::foreign_data::ForeignData;
use super::identifiers::Identifiers;
use super::leadership_skills::LeadershipSkills;
use super::legalities::Legalities;
use super::purchase_urls::PurchaseURLs;
use super::ruling::Ruling;

// This might be used in the future
// use crate::utils::number_deserializer::deserialize_number_from_string;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use core::fmt;

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct AtomicCardFace {
    uuid: Option<String>,
    artist: Option<String>,
    asciiName: Option<String>,
    availability: Option<Vec<String>>,
    borderColor: Option<String>,
    colorIdentity: Vec<String>,
    colorIndicator: Option<Vec<String>>,
    colors: Vec<String>,
    convertedManaCost: Option<f32>,
    count: Option<u32>,
    duelDeck: Option<String>,
    edhrecRank: Option<i32>,
    faceConvertedManaCost: Option<f32>,
    faceName: Option<String>,
    flavorName: Option<String>,
    flavorText: Option<String>,
    foreignData: Option<Vec<ForeignData>>,
    frameEffects: Option<Vec<String>>,
    frameVersion: Option<String>,
    hand: Option<String>,
    hasAlternativeDeckLimit: Option<bool>,
    hasContentWarning: Option<bool>,
    hasFoil: Option<bool>,
    hasNonFoil: Option<bool>,
    identifiers: Option<Identifiers>,
    isAlternative: Option<bool>,
    isFullArt: Option<bool>,
    isOnlineOnly: Option<bool>,
    isOversized: Option<bool>,
    isPromo: Option<bool>,
    isReprint: Option<bool>,
    isReserved: Option<bool>,
    isStarter: Option<bool>,
    isStorySpotlight: Option<bool>,
    isTextless: Option<bool>,
    isTimeshifted: Option<bool>,
    isFoil: Option<bool>,
    keywords: Option<Vec<String>>,
    pub layout: String,
    legalities: Option<Legalities>,
    leadershipSkills: Option<LeadershipSkills>,
    life: Option<String>,
    loyalty: Option<String>,
    pub manaCost: Option<String>,
    pub name: String,
    number: Option<String>,
    otherFaceIds: Option<Vec<String>>,
    originalReleaseDate: Option<String>,
    originalText: Option<String>,
    originalType: Option<String>,
    purchaseUrls: Option<PurchaseURLs>,
    power: Option<String>,
    printings: Option<Vec<String>>,
    promoTypes: Option<Vec<String>>,
    rarity: Option<String>,
    reverseRelated: Option<Vec<String>>,
    rulings: Option<Vec<Ruling>>,
    setCode: Option<String>,
    side: Option<String>,
    subtypes: Vec<String>,
    supertypes: Vec<String>,
    text: Option<String>,
    toughness: Option<String>,
    types: Vec<String>,
    variations: Option<Vec<String>>,
    watermark: Option<Vec<String>>,
}

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct AtomicCard {
    pub faces: Vec<AtomicCardFace>,
}

impl AtomicCard {}

impl fmt::Display for AtomicCardFace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut digest: String = self.name.clone();
        digest += "\n";
        if self.text.is_some() {
            digest += &self.text.as_ref().unwrap();
        }
        write!(f, "{}", digest)
    }
}

impl fmt::Display for AtomicCard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut digest: String = String::from("Front Face: ");
        digest += &self.faces[0].name;
        if self.faces.len() == 2 {
            digest += "Back Face: ";
            digest += &self.faces[1].name;
        }
        digest += "\n";
        write!(f, "{}", digest)
    }
}
