use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PurchaseURLs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_kingdom: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_kingdom_foil: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_market: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcg_player: Option<String>,
}
