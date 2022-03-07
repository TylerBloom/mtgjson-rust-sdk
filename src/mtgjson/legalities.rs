use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
/*
use std::str::FromStr;
use strum_macros::Display;

#[non_exhaustive]
#[derive(Display)]
pub enum Legality {
    legal,
    banned,
    not_legal,
    restricted,
    suspended,
    unknown,
}
impl FromStr for Legality {
    type Err = ();

    fn from_str(input: &str) -> Result<Legality, Self::Err> {
        match input {
            "legal" => Ok(Legality::legal),
            "banned" => Ok(Legality::banned),
            "not_legal" => Ok(Legality::not_legal),
            "not-legal" => Ok(Legality::not_legal),
            "not legal" => Ok(Legality::not_legal),
            "restricted" => Ok(Legality::restricted),
            "suspended" => Ok(Legality::suspended),
            _ => Ok(Legality::unknown),
        }
    }
}
*/

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Legalities {
    pub brawl: Option<String>,
    pub duel: Option<String>,
    pub frontier: Option<String>,
    pub legacy: Option<String>,
    pub pauper: Option<String>,
    pub pioneer: Option<String>,
    pub vintage: Option<String>,
    pub commander: Option<String>,
    pub future: Option<String>,
    pub historic: Option<String>,
    pub modern: Option<String>,
    pub penny: Option<String>,
    pub standard: Option<String>,
}
