use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct ForeignData {
    faceName: Option<String>,
    flavorText: Option<String>,
    language: Option<String>,
    multiverseId: Option<u32>,
    name: Option<String>,
    text: Option<String>,
    types: Option<String>,
}
