use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LeadershipSkills {
    brawl: bool,
    commander: bool,
    oathbreaker: bool,
}
