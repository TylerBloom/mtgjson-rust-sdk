use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Identifiers {
    #[serde(skip_serializing_if = "Option::is_none")]
    cardKingdomFoilId: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cardKingdomId: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mcmId: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mcmMetaId: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mtgArenaId: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mtgoFoilId: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mtgoId: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mtgjsonV4Id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    multiverseId: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scryfallId: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scryfallOracleId: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scryfallIllustrationId: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tcgplayerProductId: Option<String>,
}
