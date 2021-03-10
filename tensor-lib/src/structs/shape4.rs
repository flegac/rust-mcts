use std::fmt::{Display, Formatter};
use std::fmt;

use crate::structs::dim::Dim;
use crate::structs::dim::Dim::Size;
use crate::structs::offset4::Offset4;

pub const NDIMS: usize = 4;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Shape4(Dim, Dim, Dim, Dim);

impl Shape4 {
    pub fn vec4(x: usize, y: usize, z: usize, t: usize) -> Shape4 {
        Shape4(Size(x), Size(y), Size(z), Size(t))
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
}

impl Shape4 {
    #[inline]
    pub fn x(&self) -> Dim {
        self.0
    }
    #[inline]
    pub fn y(&self) -> Dim {
        self.1
    }
    #[inline]
    pub fn z(&self) -> Dim {
        self.2
    }
    #[inline]
    pub fn t(&self) -> Dim {
        self.3
    }
    #[inline]
    pub fn volume(&self) -> Dim {
        self.x() * self.y() * self.z() * self.t()
    }
    #[inline]
    pub fn len(&self) -> usize {
        self.volume().unwrap()
    }
}

impl Shape4 {
    pub fn offset4(&self, offset: usize) -> Offset4 {
        Offset4::from_shape(self, offset)
    }

    pub fn index(&self, offset: &Offset4) -> usize {
        log::trace!("Shape4.index");
        offset.index_from(self)
    }
    pub fn check(&self, offset: &Offset4) {
        self.0.check(offset.0);
        self.1.check(offset.1);
        self.2.check(offset.2);
        self.3.check(offset.3);
    }
}

impl Display for Shape4 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}x{}x{}x{}", self.0, self.1, self.2, self.3)
    }
}

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use crate::structs::offset4::Offset4;
    use crate::structs::shape4::Shape4;

    #[test]
    fn test_mem_size() {
        println!("Offset4 : {}", size_of::<Offset4>());
        println!("&Offset4: {}", size_of::<&Offset4>());
        println!("Shape4  : {}", size_of::<Shape4>());
        println!("&Shape4 : {}", size_of::<&Shape4>());
    }

    #[test]
    fn test_new() {
        let x = 2;
        let y = 3;
        let z = 5;
        let t = 53;
        let shape = Shape4::vec4(x, y, z, t);

        println!("{} {} {} {}", x, y, z, t);
        println!("{}", shape);

        assert_eq!(x, shape.x().unwrap());
        assert_eq!(y, shape.y().unwrap());
        assert_eq!(z, shape.z().unwrap());
        assert_eq!(t, shape.t().unwrap());

        let xy = shape.x() * shape.y();
        assert_eq!(x * y, xy.unwrap());
        let yt = shape.y() * shape.t();
        assert_eq!(y * t, yt.unwrap());
    }

    #[test]
    fn test_shape() {
        let shape = Shape4::vec4(5, 3, 4, 7);


        let d0 = shape.x().unwrap();
        let d1 = shape.y().unwrap();
        let d2 = shape.z().unwrap();
        let d3 = shape.t().unwrap();
        println!("{} {} {} {}", d0, d1, d2, d3, );
        for l in 0..d3 {
            for k in 0..d2 {
                for j in 0..d1 {
                    for i in 0..d0 {
                        println!("-[{},{},{},{}]----------------------------------------------------", i, j, k, l);
                        let coord1 = Offset4::new(i, j, k, l);
                        let index1 = shape.index(&coord1);
                        let coord2 = shape.offset4(index1);
                        println!("{:?} -> {} -> {:?}", coord1, index1, coord2);
                        let id2 = shape.index(&coord2);
                        assert_eq!(index1, id2);
                        println!("-----------------------------------------");
                    }
                }
            }
        }
    }
}
