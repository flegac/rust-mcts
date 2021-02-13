use std::fmt;
use std::fmt::Formatter;

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub enum Stone {
    None,
    Black,
    White,
}

impl Stone {
    pub fn switch(&self) -> Self {
        match self {
            Stone::Black => Stone::White,
            Stone::White => Stone::Black,
            Stone::None => Stone::None
        }
    }
}

impl fmt::Display for Stone {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Stone::Black => "*",
            Stone::White => "o",
            Stone::None => "."
        })
    }
}