use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Identifiers {
    cardKingdomFoilId: Option<String>,
    cardKingdomId: Option<String>,
    mcmId: Option<String>,
    mcmMetaId: Option<String>,
    mtgArenaId: Option<String>,
    mtgoFoilId: Option<String>,
    mtgoId: Option<String>,
    mtgjsonV4Id: Option<String>,
    multiverseId: Option<String>,
    scryfallId: Option<String>,
    scryfallOracleId: Option<String>,
    scryfallIllustrationId: Option<String>,
    tcgplayerProductId: Option<String>,
}
