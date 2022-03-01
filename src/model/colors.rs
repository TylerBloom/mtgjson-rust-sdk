
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Colors {
    White,
    Blue,
    Black,
    Red,
    Green,
}
