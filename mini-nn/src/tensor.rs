use std::borrow::Borrow;
use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::fmt;
use std::ops::{Add, AddAssign, Deref, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use std::rc::Rc;

use crate::buffer::Buffer;
use crate::dim::Dim;
use crate::shape::{Shape, ShapeIndex};
use crate::shape4::{NDIMS, Shape4};

#[derive(Debug, Clone)]
pub struct View {
    offset: (usize, usize, usize, usize),
    pub shape: Shape4,
}

#[derive(Debug, Clone)]
pub struct Tensor {
    pub buffer: Rc<RefCell<Buffer>>,
    pub view: View,
}

impl Shape for Tensor {
    fn shape(&self) -> &[Dim; NDIMS] {
        &self.view.shape.shape()
    }
}


impl ShapeIndex for Tensor {
    fn index(&self, x: usize, y: usize, z: usize, t: usize) -> usize {
        assert!(x < self.view.shape.x().unwrap());
        assert!(y < self.view.shape.y().unwrap());
        assert!(z < self.view.shape.z().unwrap());
        assert!(t < self.view.shape.t().unwrap());
        let xx = x + self.view.offset.0;
        let yy = y + self.view.offset.1;
        let zz = z + self.view.offset.2;
        let tt = t + self.view.offset.3;
        self.buffer.as_ref().borrow().index(xx, yy, zz, tt)
    }

    fn coord(&self, id: usize) -> (usize, usize, usize, usize) {
        let (x, y, z, t) = self.view.shape.coord(id);
        (x + self.view.offset.0, y + self.view.offset.1, z + self.view.offset.2, t + self.view.offset.3)
    }
}


impl Tensor {
    pub fn new(buffer: Buffer, view: View) -> Self {
        Tensor {
            buffer: Rc::new(RefCell::new(buffer)),
            view,
        }
    }

    pub fn view(&self, offset: (usize, usize, usize, usize), shape: Shape4) -> Tensor {
        Tensor {
            buffer: self.buffer.clone(),
            view: View {
                offset,
                shape,
            },
        }
    }

    pub fn from_shape(shape: Shape4, value: f32) -> Tensor {
        Self::new(
            Buffer::new(shape, value),
            View {
                offset: (0, 0, 0, 0),
                shape,
            })
    }
}


impl Add for Tensor {
    type Output = Tensor;

    fn add(self, rhs: Self) -> Tensor {
        let mut res = self.clone();
        res += rhs;
        res
    }
}

impl AddAssign for Tensor {
    fn add_assign(&mut self, rhs: Self) {
        assert_eq!(self.buffer.as_ref().borrow().shape, rhs.buffer.as_ref().borrow().shape);
        let buffer = &mut self.buffer.borrow_mut().data;
        let other = &rhs.buffer.as_ref().borrow().data;
        for i in 0..other.len() {
            buffer[i] += other[i];
        }
    }
}

impl Sub for Tensor {
    type Output = Tensor;

    fn sub(self, rhs: Self) -> Tensor {
        let mut res = self.clone();
        res -= rhs;
        res
    }
}

impl SubAssign for Tensor {
    fn sub_assign(&mut self, rhs: Self) {
        assert_eq!(self.buffer.as_ref().borrow().shape, rhs.buffer.as_ref().borrow().shape);
        let buffer = &mut self.buffer.borrow_mut().data;
        let other = &rhs.buffer.as_ref().borrow().data;
        for i in 0..other.len() {
            buffer[i] -= other[i];
        }
    }
}

impl Mul for Tensor {
    type Output = Tensor;

    fn mul(self, rhs: Self) -> Tensor {
        let mut res = self.clone();
        res *= rhs;
        res
    }
}

impl MulAssign for Tensor {
    fn mul_assign(&mut self, rhs: Self) {
        assert_eq!(self.buffer.as_ref().borrow().shape, rhs.buffer.as_ref().borrow().shape);
        let buffer = &mut self.buffer.borrow_mut().data;
        let other = &rhs.buffer.as_ref().borrow().data;
        for i in 0..other.len() {
            buffer[i] *= other[i];
        }
    }
}


impl Div for Tensor {
    type Output = Tensor;

    fn div(self, rhs: Self) -> Tensor {
        let mut res = self.clone();
        res /= rhs;
        res
    }
}

impl DivAssign for Tensor {
    fn div_assign(&mut self, rhs: Self) {
        assert_eq!(self.buffer.as_ref().borrow().shape, rhs.buffer.as_ref().borrow().shape);
        let buffer = &mut self.buffer.borrow_mut().data;
        let other = &rhs.buffer.as_ref().borrow().data;
        for i in 0..other.len() {
            buffer[i] /= other[i];
        }
    }
}


#[cfg(test)]
mod tests {
    use std::time::Duration;

    use rust_tools::bench::Bench;

    use crate::buffer::Buffer;
    use crate::shape4::Shape4;
    use crate::tensor::Tensor;

    #[test]
    fn test_tensor() {
        let shape = Shape4::vec4(32, 32, 128, 1);
        let mut x = Tensor::from_shape(shape, 3_f32);
        let y = Tensor::from_shape(shape, 1_f32);


        let mut bench = Bench::new();
        while bench.for_duration(Duration::from_secs(3)) {
            x *= y.clone();
        }

        // println!("{:?}", x);
        println!("{}", bench);
    }
}
