use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ruling {
    pub date: String,
    pub text: String,
}
