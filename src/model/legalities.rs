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

pub struct Legalities {
    pub brawl: Legality,
    pub duel: Legality,
    pub frontier: Legality,
    pub legacy: Legality,
    pub pauper: Legality,
    pub pioneer: Legality,
    pub vintage: Legality,
    pub commander: Legality,
    pub future: Legality,
    pub historic: Legality,
    pub modern: Legality,
    pub penny: Legality,
    pub standard: Legality,
}
