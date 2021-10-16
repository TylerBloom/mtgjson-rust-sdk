use std::str::FromStr;
use strum_macros::Display;

#[non_exhaustive]
#[derive(Display)]
enum Types {
    Artifact,
    Creature,
    Land,
    Enchantment,
    Planeswalker,
    Instant,
    Sorcery,
    Tribal,
    Conspiracy,
    Phenomenon,
    Plane,
    Scheme,
    Vangaurd,
}
impl FromStr for Types {
    type Err = ();

    fn from_str(input: &str) -> Result<Types, Self::Err> {
        match input {
            "Artifact" => Ok(Types::Artifact),
            "Creature" => Ok(Types::Creature),
            "Land" => Ok(Types::Land),
            "Enchantment" => Ok(Types::Enchantment),
            "Planeswalker" => Ok(Types::Planeswalker),
            "Instant" => Ok(Types::Instant),
            "Sorcery" => Ok(Types::Sorcery),
            "Tribal" => Ok(Types::Tribal),
            "Conspiracy" => Ok(Types::Conspiracy),
            "Phenomenon" => Ok(Types::Phenomenon),
            "Plane" => Ok(Types::Plane),
            "Scheme" => Ok(Types::Scheme),
            "Vangaurd" => Ok(Types::Vangaurd),
            _ => Err(()),
        }
    }
}
