use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};
use crate::structs::offset4::Offset4;

impl<'a> Add for &'a Offset4 {
    type Output = Offset4;

    fn add(self, rhs: Self) -> Self::Output {
        let mut res = self.clone();
        res.0 += rhs.0;
        res.1 += rhs.1;
        res.2 += rhs.2;
        res.3 += rhs.3;
        res
    }
}

impl<'a> Sub for &'a Offset4 {
    type Output = Offset4;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut res = self.clone();
        res.0 -= rhs.0;
        res.1 -= rhs.1;
        res.2 -= rhs.2;
        res.3 -= rhs.3;
        res
    }
}

impl<'a> Mul for &'a Offset4 {
    type Output = Offset4;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut res = self.clone();
        res.0 *= rhs.0;
        res.1 *= rhs.1;
        res.2 *= rhs.2;
        res.3 += rhs.3;
        res
    }
}

impl<'a> Div for &'a Offset4 {
    type Output = Offset4;

    fn div(self, rhs: Self) -> Self::Output {
        let mut res = self.clone();
        res.0 /= rhs.0;
        res.1 /= rhs.1;
        res.2 /= rhs.2;
        res.3 /= rhs.3;
        res
    }
}

impl AddAssign for Offset4 {
    fn add_assign(&mut self, rhs: Offset4) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
        self.3 += rhs.3;
    }
}

impl<'a> SubAssign for &'a mut Offset4 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
        self.3 -= rhs.3;
    }
}

impl<'a> MulAssign for &'a mut Offset4 {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
        self.1 *= rhs.1;
        self.2 *= rhs.2;
        self.3 *= rhs.3;
    }
}

impl<'a> DivAssign for &'a mut Offset4 {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
        self.1 /= rhs.1;
        self.2 /= rhs.2;
        self.3 /= rhs.3;
    }
}

