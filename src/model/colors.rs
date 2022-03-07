use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Colors {
    White,
    Blue,
    Black,
    Red,
    Green,
}

impl Colors {
    pub fn from_str(c: &str) -> Option<Self> {
        let color = c.to_lowercase();
        match color.as_str() {
            "w" | "{w}" | "white" => Some(Colors::White),
            "u" | "{u}" | "blue" => Some(Colors::Blue),
            "b" | "{b}" | "black" => Some(Colors::Black),
            "r" | "{r}" | "red" => Some(Colors::Red),
            "g" | "{g}" | "green" => Some(Colors::Green),
            _ => None,
        }
    }
}
