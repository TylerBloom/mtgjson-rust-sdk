use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct PurchaseURLs {
    pub card_kingdom: Option<String>,
    pub card_kingdom_foil: Option<String>,
    pub card_market: Option<String>,
    pub tcg_player: Option<String>,
}
