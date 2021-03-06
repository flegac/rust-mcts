use std::fmt::{Display, Formatter};
use std::fmt;
use std::iter::Product;
use std::ops::{Add, Div, Mul, Sub};
use crate::structs::dim::Dim::Size;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Dim {
    Any,
    Size(usize),
}

impl Display for Dim {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let res = match self {
            Dim::Any => String::from("Any"),
            Dim::Size(v) => format!("{}", v)
        };
        write!(f, "{}", res)
    }
}

impl Dim {
    pub fn unwrap(&self) -> usize {
        match self {
            Dim::Any => panic!(),
            Dim::Size(value) => value.clone()
        }
    }
    pub fn check(&self, offset: usize) {
        match self {
            Dim::Any => {}
            Dim::Size(size) => assert!(offset < *size)
        }
    }
}

impl Product for Dim {
    fn product<I: Iterator<Item=Dim>>(iter: I) -> Self {
        iter.fold(Size(1), |a, b| a * b)
    }
}

impl Add for Dim {
    type Output = Dim;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Dim::Any => Dim::Any,
            Size(v1) => {
                match rhs {
                    Dim::Any => Dim::Any,
                    Size(v2) => Size(v1 + v2)
                }
            }
        }
    }
}

impl Sub for Dim {
    type Output = Dim;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Dim::Any => Dim::Any,
            Size(v1) => {
                match rhs {
                    Dim::Any => Dim::Any,
                    Size(v2) => Size(v1 - v2)
                }
            }
        }
    }
}

impl Mul for Dim {
    type Output = Dim;
    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Dim::Any => Dim::Any,
            Size(v1) => {
                match rhs {
                    Dim::Any => Dim::Any,
                    Size(v2) => {
                        Size(v1 * v2)
                    }
                }
            }
        }
    }
}

impl Div for Dim {
    type Output = Dim;
    fn div(self, rhs: Self) -> Self::Output {
        match self {
            Dim::Any => Dim::Any,
            Size(v1) => {
                match rhs {
                    Dim::Any => Dim::Any,
                    Size(v2) => Size(v1 / v2)
                }
            }
        }
    }
}

impl Sub<Dim> for usize {
    type Output = Dim;

    fn sub(self, rhs: Dim) -> Self::Output {
        match rhs {
            Dim::Any => Dim::Any,
            Size(v) => {
                Size(self - v)
            }
        }
    }
}


impl Div<Dim> for usize {
    type Output = Dim;

    fn div(self, rhs: Dim) -> Self::Output {
        match rhs {
            Dim::Any => Dim::Any,
            Size(v) => {
                Size(self / v)
            }
        }
    }
}
