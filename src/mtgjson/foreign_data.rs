use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
pub struct ForeignData {
    pub faceName: Option<String>,
    pub flavorText: Option<String>,
    pub language: Option<String>,
    pub multiverseId: Option<u32>,
    pub name: Option<String>,
    pub text: Option<String>,
    pub r#type: Option<String>,
}
