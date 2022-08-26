//use crate::model::minimal_card::{MinimalCard, MinimalCardFace};

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
use std::{
    collections::HashSet,
    hash::{Hash, Hasher},
};

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AtomicCardFace {
    uuid: Option<String>,
    artist: Option<String>,
    asciiName: Option<String>,
    availability: Option<Vec<String>>,
    borderColor: Option<String>,
    colorIdentity: Vec<String>,
    pub colorIndicator: Option<Vec<String>>,
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
    pub hand: Option<String>,
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
    pub life: Option<String>,
    pub loyalty: Option<String>,
    pub manaCost: Option<String>,
    pub name: String,
    number: Option<String>,
    otherFaceIds: Option<Vec<String>>,
    originalReleaseDate: Option<String>,
    originalText: Option<String>,
    originalType: Option<String>,
    purchaseUrls: Option<PurchaseURLs>,
    pub power: Option<String>,
    printings: Option<Vec<String>>,
    promoTypes: Option<Vec<String>>,
    rarity: Option<String>,
    reverseRelated: Option<Vec<String>>,
    rulings: Option<Vec<Ruling>>,
    setCode: Option<String>,
    side: Option<String>,
    pub subtypes: Vec<String>,
    pub supertypes: Vec<String>,
    pub text: Option<String>,
    pub toughness: Option<String>,
    pub types: Vec<String>,
    pub r#type: String,
    variations: Option<Vec<String>>,
    watermark: Option<Vec<String>>,
}

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
#[serde(transparent)]
pub struct AtomicCard {
    pub faces: Vec<AtomicCardFace>,
}

impl AtomicCard {
    pub fn get_names(&self) -> HashSet<String> {
        let mut digest: HashSet<String> = self
            .faces
            .iter()
            .filter_map(|f| f.faceName.clone())
            .collect();
        digest.insert(self.faces[0].name.clone());
        if let Some(name) = self
            .faces
            .iter()
            .filter_map(|f| f.faceName.clone())
            .reduce(|a, b| format!("{} / {}", a, b))
        {
            digest.insert(name);
        }
        if let Some(name) = self
            .faces
            .iter()
            .filter_map(|f| f.faceName.clone())
            .reduce(|a, b| format!("{} // {}", a, b))
        {
            digest.insert(name);
        }
        /*
        if self.faces[0].name.contains("Okaun, Eye of Chaos") {
            println!("{:?}", self);
        }
        */
        digest
    }

    /*
    // Option is returned if the language can't be found
    pub fn as_minimal(&self, lang: &String) -> Option<MinimalCard> {
        let name = self
            .faces
            .first()?
            .foreignData
            .as_ref()?
            .iter()
            .find_map(|f| {
                if f.language.as_ref()? == lang {
                    f.name.clone()
                } else {
                    None
                }
            })
            .unwrap_or_else(|| self.faces.first().unwrap().name.clone());
        let idents = self.faces.first()?.identifiers.as_ref()?;
        let oracle_id = idents
            .scryfallOracleId
            .clone()
            .unwrap_or_else(|| idents.scryfallId.clone().unwrap());
        let mut faces = Vec::with_capacity(self.faces.len());
        for face in self.faces.iter() {
            faces.push(face.as_minimal(lang)?);
        }
        Some(MinimalCard {
            name,
            oracle_id,
            faces,
        })
    }
    */
}

impl AtomicCardFace {
    /*
    // Options is returned if the language can't be found
    pub fn as_minimal(&self, lang: &String) -> Option<MinimalCardFace> {
        let face_name = self
            .foreignData
            .as_ref()?
            .iter()
            .find_map(|f| {
                if f.language.as_ref()? == lang {
                    if let Some(name) = f.faceName.as_ref() {
                        Some(name.clone())
                    } else {
                        f.name.clone()
                    }
                } else {
                    None
                }
            })
            .unwrap_or_else(|| self.faceName.clone().unwrap_or_else(|| self.name.clone()));
        let text = self
            .foreignData
            .as_ref()?
            .iter()
            .find_map(|f| {
                if f.language.as_ref()? == lang {
                    Some(f.text.clone().unwrap_or_default())
                } else {
                    None
                }
            })
            .unwrap_or_else(|| self.text.clone().unwrap_or_default());
        let types = self
            .foreignData
            .as_ref()?
            .iter()
            .find_map(|f| {
                if f.language.as_ref()? == lang {
                    f.r#type.clone()
                } else {
                    None
                }
            })
            .unwrap_or_else(|| self.r#type.clone());
        Some(MinimalCardFace {
            face_name,
            text,
            types,
        })
    }
    */
}

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

impl Hash for AtomicCardFace {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for AtomicCardFace {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for AtomicCardFace {}

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
