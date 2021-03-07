use std::fmt::{Display, Formatter};
use std::fmt;

use crate::structs::shape4::Shape4;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Offset4(pub usize, pub usize, pub usize, pub usize);

impl Offset4 {
    pub fn new(x: usize, y: usize, z: usize, t: usize) -> Self {
        Offset4(x, y, z, t)
    }

    pub fn origin() -> Offset4 {
        Offset4(0, 0, 0, 0)
    }

    pub fn from_shape(shape: &Shape4, offset: usize) -> Offset4 {
        let s0 = shape.x().unwrap();
        let s1 = shape.y().unwrap();
        let s2 = shape.z().unwrap();

        let v1 = s0 * s1;
        let v2 = v1 * s2;

        let mut id = offset;
        let t = id / v2;
        id = id - t * v2;
        let z = id / v1;
        id = id - z * v1;
        let y = id / s0;
        id = id - y * s0;
        let x = id;
        Offset4(x, y, z, t)
    }

    pub fn index_from(&self, shape: &Shape4) -> usize {
        let mut res = self.x();
        let mut k = shape.x().unwrap();
        res = res + (self.y() * k);
        k *= shape.y().unwrap();
        res = res + (self.z() * k);
        k *= shape.z().unwrap();
        res = res + (self.t() * k);
        res
    }
}

impl Offset4 {
    #[inline]
    pub fn x(&self) -> usize {
        self.0
    }
    #[inline]
    pub fn y(&self) -> usize {
        self.1
    }
    #[inline]
    pub fn z(&self) -> usize {
        self.2
    }
    #[inline]
    pub fn t(&self) -> usize {
        self.3
    }
}

impl Display for Offset4 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({},{},{},{})", self.0, self.1, self.2, self.3)
    }
}