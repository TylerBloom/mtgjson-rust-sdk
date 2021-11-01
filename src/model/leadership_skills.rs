use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct LeadershipSkills {
    brawl: bool,
    commander: bool,
    oathbreaker: bool,
}
