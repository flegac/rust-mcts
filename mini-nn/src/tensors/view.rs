use std::fmt::{Display, Formatter};
use std::fmt;

use crate::tensors::shape4::Shape4;

#[derive(Debug, Clone)]
pub struct View {
    pub offset: (usize, usize, usize, usize),
    pub shape: Shape4,
}


impl Display for View {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.offset)
    }
}