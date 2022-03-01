use super::card_attribute::CardFace;
use crate::mtgjson::card::AtomicCard;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbstractCard {
    front_face: CardFace,
    back_face: Option<CardFace>,
}

impl<C> From<C> for AbstractCard
where
    C: AsRef<AtomicCard>,
{
    fn from(card: C) -> Self {
        let ref_card = card.as_ref();
        let has_bottom_half = match ref_card.faces[0].layout.as_str() {
            "adventure" | "aftermath" | "flip" | "split" => true,
            _ => false,
        };
        todo!()
    }
}
