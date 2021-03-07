use crate::structs::offset4::Offset4;
use crate::structs::shape4::Shape4;
use crate::traits::view::View;
use std::fmt::{Display, Formatter};
use std::fmt;

#[derive(Debug, Clone)]
pub struct View4 {
    pub offset: Offset4,
    pub shape: Shape4,
}

impl View4 {
    pub fn new(shape: Shape4) -> Self {
        View4 {
            offset: Offset4::origin(),
            shape,
        }
    }
}

impl View for View4 {
    fn offset(&self) -> &Offset4 {
        &self.offset
    }

    fn shape(&self) -> &Shape4 {
        &self.shape
    }
}


impl Display for View4 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.offset)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_view() {}
}
