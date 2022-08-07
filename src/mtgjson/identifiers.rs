use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
pub struct Identifiers {
    pub cardKingdomFoilId: Option<String>,
    pub cardKingdomId: Option<String>,
    pub mcmId: Option<String>,
    pub mcmMetaId: Option<String>,
    pub mtgArenaId: Option<String>,
    pub mtgoFoilId: Option<String>,
    pub mtgoId: Option<String>,
    pub mtgjsonV4Id: Option<String>,
    pub multiverseId: Option<String>,
    pub scryfallId: Option<String>,
    pub scryfallOracleId: Option<String>,
    pub scryfallIllustrationId: Option<String>,
    pub tcgplayerProductId: Option<String>,
}
