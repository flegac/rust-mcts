use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use crate::tensor::Tensor;
use crate::traits::view::View;


impl<'a> Add for &'a Tensor {
    type Output = Tensor;

    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.shape(), rhs.shape());
        let mut res = self.clone();
        for i in 0..res.shape().len() {
            let val = res.get(i);
            res.insert(i, val + rhs.get(i));
        }
        res
    }
}

impl<'a> Sub for &'a Tensor {
    type Output = Tensor;

    fn sub(self, rhs: Self) -> Self::Output {
        assert_eq!(self.shape(), rhs.shape());
        let mut res = self.clone();
        for i in 0..res.shape().len() {
            let val = res.get(i);
            res.insert(i, val - rhs.get(i));
        }
        res
    }
}

impl<'a> Mul for &'a Tensor {
    type Output = Tensor;

    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.shape(), rhs.shape());
        let mut res = self.clone();
        for i in 0..res.shape().len() {
            let val = res.get(i);
            res.insert(i, val * rhs.get(i));
        }
        res
    }
}

impl<'a> Div for &'a Tensor {
    type Output = Tensor;

    fn div(self, rhs: Self) -> Self::Output {
        assert_eq!(self.shape(), rhs.shape());
        let mut res = self.clone();
        for i in 0..res.shape().len() {
            let val = res.get(i);
            res.insert(i, val / rhs.get(i));
        }
        res
    }
}


impl AddAssign for Tensor {
    fn add_assign(&mut self, rhs: Self) {
        assert_eq!(self.shape(), rhs.shape());
        for i in 0..self.shape().len() {
            let val = self.get(i);
            self.insert(i, val + rhs.get(i));
        }
    }
}


impl SubAssign for Tensor {
    fn sub_assign(&mut self, rhs: Self) {
        assert_eq!(self.shape(), rhs.shape());
        for i in 0..self.shape().len() {
            let val = self.get(i);
            self.insert(i, val - rhs.get(i));
        }
    }
}

impl MulAssign for Tensor {
    fn mul_assign(&mut self, rhs: Self) {
        assert_eq!(self.shape(), rhs.shape());
        for i in 0..self.shape().len() {
            let val = self.get(i);
            self.insert(i, val * rhs.get(i));
        }
    }
}


impl DivAssign for Tensor {
    fn div_assign(&mut self, rhs: Self) {
        assert_eq!(self.shape(), rhs.shape());
        for i in 0..self.shape().len() {
            let val = self.get(i);
            self.insert(i, val / rhs.get(i));
        }
    }
}
