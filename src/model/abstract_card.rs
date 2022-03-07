use super::card_attribute::CardFace;
use crate::mtgjson::card::AtomicCard;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbstractCard {
    front_face: CardFace,
    back_face: Option<CardFace>,
}

impl From<&AtomicCard> for AbstractCard {
    fn from(card: &AtomicCard) -> Self {
        let mut front_face = CardFace::from(&card.faces[0]);
        let mut back_face: Option<CardFace> = None;
        if card.faces.len() == 2 {
            match card.faces[0].layout.as_str() {
                "adventure" | "aftermath" | "flip" | "split" => {
                    let bottom_half = CardFace::from(&card.faces[1]);
                    front_face.add_bottom_half(bottom_half);
                }
                _ => {
                    back_face = Some(CardFace::from(&card.faces[1]));
                }
            };
        }

        AbstractCard {
            front_face,
            back_face,
        }
    }
}
