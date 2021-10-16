use std::str::FromStr;
use strum_macros::Display;

#[non_exhaustive]
#[derive(Display)]
enum SuperTypes {
    Basic,
    Legendary,
    Ongoing,
    Snow,
    World,
}

impl FromStr for SuperTypes {
    type Err = ();

    fn from_str(input: &str) -> Result<SuperTypes, Self::Err> {
        match input {
            "Basic" => Ok(SuperTypes::Basic),
            "Legendary" => Ok(SuperTypes::Legendary),
            "Ongoing" => Ok(SuperTypes::Ongoing),
            "Snow" => Ok(SuperTypes::Snow),
            "World" => Ok(SuperTypes::World),
            _ => Err(()),
        }
    }
}
