use std::fmt::{Display, Formatter};
use std::fmt;

use crate::tensors::dim::Dim;
use crate::tensors::dim::Dim::Size;
use crate::tensors::shape::{Shape, ShapeIndex};

pub const NDIMS: usize = 4;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Shape4 {
    pub(crate) shape: [Dim; NDIMS]
}

impl Shape for Shape4 {
    #[inline]
    fn shape(&self) -> &[Dim; NDIMS] {
        &self.shape
    }
}

impl ShapeIndex for Shape4 {}

impl Display for Shape4 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}x{}x{}x{}", self.shape[0], self.shape[1], self.shape[2], self.shape[3])
    }
}

impl Shape4 {
    pub fn vec4(x: usize, y: usize, z: usize, t: usize) -> Shape4 {
        Self::new(Size(x), Size(y), Size(z), Size(t))
    }
    pub fn vec3(x: usize, y: usize, z: usize) -> Shape4 {
        Self::vec4(x, y, z, 1)
    }
    pub fn vec2(x: usize, y: usize) -> Shape4 {
        Self::vec4(x, y, 1, 1)
    }
    pub fn vec1(x: usize) -> Shape4 {
        Self::vec4(x, 1, 1, 1)
    }

    pub fn new(x: Dim, y: Dim, z: Dim, t: Dim) -> Self {
        Shape4 { shape: [x, y, z, t] }
    }
}

#[cfg(test)]
mod tests {
    use crate::tensors::shape::{Shape, ShapeIndex};
    use crate::tensors::shape4::Shape4;

    #[test]
    fn test_new() {
        let x = 2;
        let y = 3;
        let z = 5;
        let t = 53;
        let shape = Shape4::vec4(x, y, z, t);

        println!("{} {} {} {}", x, y, z, t);
        println!("{}", shape);

        assert_eq!(x, shape.shape[0].unwrap());
        assert_eq!(y, shape.shape[1].unwrap());
        assert_eq!(z, shape.shape[2].unwrap());
        assert_eq!(t, shape.shape[3].unwrap());

        let xy = shape.shape[0] * shape.shape[1];
        assert_eq!(x * y, xy.unwrap());
        let yt = shape.shape[1] * shape.shape[3];
        assert_eq!(y * t, yt.unwrap());
    }

    #[test]
    fn test_shape() {
        let shape = Shape4::vec4(5, 3, 4, 7);


        let d0 = shape.shape()[0].unwrap();
        let d1 = shape.shape()[1].unwrap();
        let d2 = shape.shape()[2].unwrap();
        let d3 = shape.shape()[3].unwrap();
        println!("{} {} {} {}", d0, d1, d2, d3, );
        for l in 0..d3 {
            for k in 0..d2 {
                for j in 0..d1 {
                    for i in 0..d0 {
                        println!("-[{},{},{},{}]----------------------------------------------------", i, j, k, l);
                        let coord1 = Shape4::vec4(i, j, k, l);
                        let id1 = shape.index(i, j, k, l);
                        let coord2 = shape.coord(id1);
                        println!("{} -> {} -> {:?}", coord1, id1, coord2);
                        let (x, y, z, t) = coord2;
                        let id2 = shape.index(x, y, z, t);
                        assert_eq!(id1, id2);
                        println!("-----------------------------------------");
                    }
                }
            }
        }
    }
}
