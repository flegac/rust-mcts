use std::fmt;
use std::fmt::Formatter;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Stone {
    Black,
    White,
}

impl Stone {
    pub fn switch(&self) -> Self {
        match self {
            Stone::Black => {
                Stone::White
            }
            Stone::White => {
                Stone::Black
            }
        }
    }
}

impl fmt::Display for Stone {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Stone::Black => {
                "*"
            }
            Stone::White => {
                "o"
            }
        })
    }
}