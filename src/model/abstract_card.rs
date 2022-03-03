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
        let mut front_face = CardFace::from(ref_card.faces[0]);
        let mut back_face: Option<CardFace> = None;
        if ref_card.faces.len() {
            match ref_card.faces[0].layout.as_str() {
                "adventure" | "aftermath" | "flip" | "split" => {
                    let bottom_half = CardFace::from(ref_card.faces[1]);
                    front_face.add_bottom_half(BottomHalf);
                }
                _ => {
                    back_face = Some(CardFace::from(ref_card.faces[1]));
                }
            };
        }
        
        AbstractCard {
            front_face,
            back_face,
        }
    }
}
