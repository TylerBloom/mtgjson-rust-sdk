use super::card_attribute::CardFace;

use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbstractCard {
    front_face: CardFace,
    back_face: Option<CardFace>,
}
