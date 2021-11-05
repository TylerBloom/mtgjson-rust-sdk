use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none};

// This might be used in the future
// use crate::utils::number_deserializer::deserialize_number_from_string;

use super::ruling::Ruling;
use super::foreign_data::ForeignData;
use super::leadership_skills::LeadershipSkills;
use super::identifiers::Identifiers;
use super::legalities::Legalities;
use super::purchase_urls::PurchaseURLs;

#[allow(non_snake_case)]
#[skip_serializing_none]
#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct CardFace {
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
    layout: String,
    legalities: Option<Legalities>,
    leadershipSkills: Option<LeadershipSkills>,
    life: Option<String>,
    loyalty: Option<String>,
    manaCost: Option<String>,
    name: String,
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
#[serde(transparent)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Card {
    faces: Vec<CardFace>,
}

impl ToString for CardFace {
    fn to_string( &self ) -> String {
        let mut digest: String = self.name.clone();
        digest += "\n";
        if self.text.is_some() {
            digest += &self.text.clone().unwrap().clone();
        }
        digest
    }
}

impl ToString for Card {
    fn to_string( &self ) -> String {
        let mut digest: String = String::from("Front Face: ");
        digest += &self.faces[0].name.clone();
        if self.faces.len() == 2{
            digest += "Back Face: ";
            digest += &self.faces[1].text.clone().unwrap().clone();
        }
        digest += "\n";
        digest
    }
}
