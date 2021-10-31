use serde::{Deserialize, Serialize};
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
#[derive(Serialize, Deserialize)]
pub struct Legalities {
    pub brawl: String,
    pub duel: String,
    pub frontier: String,
    pub legacy: String,
    pub pauper: String,
    pub pioneer: String,
    pub vintage: String,
    pub commander: String,
    pub future: String,
    pub historic: String,
    pub modern: String,
    pub penny: String,
    pub standard: String,
}
