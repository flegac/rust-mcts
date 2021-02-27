#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
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
