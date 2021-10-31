use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::fmt;

use super::ruling::Ruling;
use super::foreign_data::ForeignData;
use super::leadership_skills::LeadershipSkills;
use super::identifiers::Identifiers;
use super::legalities::Legalities;
use super::purchase_urls::PurchaseURLs;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Card {
    uuid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    artist: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    asciiName: Option<String>,
    availability: Vec<String>,
    borderColor: String,
    colorIdentity: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    colorIndicator: Option<Vec<String>>,
    colors: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    convertedManaCost: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duelDeck: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    faceConvertedManaCost: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    faceName: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flavorName: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flavorText: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    foreignData: Option<ForeignData>,
    frameEffects: Vec<String>,
    frameVersion: String,
    hasFoil: bool,
    hasNonFoil: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    identifiers: Option<Identifiers>,
    #[serde(skip_serializing_if = "Option::is_none")]
    isAlternative: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    isFullArt: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    isOnlineOnly: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    isOversized: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    isPromo: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    isReprint: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    isReserved: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    isStarter: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    isStorySpotlight: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    isTextless: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    isTimeshifted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    isFoil: Option<bool>,
    layout: String,
    legalities: Legalities,
    name: String,
    number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    otherFaceIds: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    purchaseUrls: Option<PurchaseURLs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rarity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rulings: Option<Vec<Ruling>>,
    setCode: String,
    subtypes: Vec<String>,
    supertypes: Vec<String>,
    types: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    variations: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reverseRelated: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    edhrecRank: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hand: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hasContentWarning: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hasAlternativeDeckLimit: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    keywords: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    leadershipSkills: Option<LeadershipSkills>,
    #[serde(skip_serializing_if = "Option::is_none")]
    life: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    loyalty: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    manaCost: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    originalReleaseDate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    originalText: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    originalType: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    power: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    printings: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    promoTypes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    side: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    toughness: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    watermark: Option<Vec<String>>,
}

impl ToString for Card {
    fn to_string( &self ) -> String {
        let mut digest: String = self.name.clone();
        digest += "\n";
        if self.text.is_some() {
            digest += &self.text.clone().unwrap().clone();
        }
        digest
    }
}
