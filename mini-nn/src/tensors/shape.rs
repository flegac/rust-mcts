use crate::tensors::dim::Dim;
use crate::tensors::shape4::NDIMS;

pub trait Shape {
    fn shape(&self) -> &[Dim; NDIMS];
    #[inline]
    fn x(&self) -> Dim {
        self.shape()[0]
    }
    #[inline]
    fn y(&self) -> Dim {
        self.shape()[1]
    }
    #[inline]
    fn z(&self) -> Dim {
        self.shape()[2]
    }
    #[inline]
    fn t(&self) -> Dim {
        self.shape()[3]
    }
    #[inline]
    fn volume(&self) -> Dim {
        self.x() * self.y() * self.z() * self.t()
    }
}

pub trait ShapeIndex where Self: Shape {
    fn coord(&self, id: usize) -> (usize, usize, usize, usize) {
        let s0 = self.x().unwrap();
        let s1 = self.y().unwrap();
        let s2 = self.z().unwrap();

        let v1 = s0 * s1;
        let v2 = v1 * s2;


        let mut id = id;
        let t = id / v2;
        id = id - t * v2;
        let z = id / v1;
        id = id - z * v1;
        let y = id / s0;
        id = id - y * s0;
        let x = id;
        (x, y, z, t)
    }

    fn index(&self, x: usize, y: usize, z: usize, t: usize) -> usize {
        let mut res = 0;
        let mut k = 1;
        for (i, &v) in [x, y, z, t].iter().enumerate() {
            // println!("res:{} k:{}", res, k);
            res = res + (v * k);
            k = k * self.shape()[i].unwrap()
        }
        // println!("res:{} k:{}", res, k);
        res
    }
}

//
// impl<T: Shape> ShapeIndex for T {
//     fn coord(&self, id: usize) -> (usize,usize,usize,usize) {
//         let s0 = self.x().unwrap();
//         let s1 = self.y().unwrap();
//         let s2 = self.z().unwrap();
//
//         let v1 = (s0 * s1);
//         let v2 = (v1 * s2);
//
//
//         let mut id = id;
//         let t = id / v2;
//         id = id - t * v2;
//         let z = id / v1;
//         id = id - z * v1;
//         let y = id / s0;
//         id = id - y * s0;
//         let x = id;
//        (x, y, z, t)
//     }
//
//     fn index(&self, x:usize,y:usize,z:usize,t:usize) -> usize {
//         let mut res = 0;
//         let mut k = 1;
//         for (i,&v) in [x,y,z,t].iter().enumerate() {
//             // println!("res:{} k:{}", res, k);
//             res = res + (v * k);
//             k = (k * self.shape()[i].unwrap())
//         }
//         // println!("res:{} k:{}", res, k);
//         res
//     }
// }

