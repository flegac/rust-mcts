use crate::tensors::dim::Dim;
use crate::tensors::shape::{Shape, ShapeIndex};
use crate::tensors::shape4::Shape4;

#[derive(Debug, Clone)]
pub struct Buffer {
    pub shape: Shape4,
    pub data: Vec<f32>,
}

impl Buffer {
    pub fn from_data(data: Vec<f32>, shape: Shape4) -> Buffer {
        Buffer {
            data,
            shape,
        }
    }

    pub fn new(shape: Shape4, value: f32) -> Self {
        match shape.volume() {
            Dim::Any => panic!(),
            Dim::Size(volume) => Buffer {
                shape,
                data: vec![value; volume],
            }
        }
    }
}

impl ShapeIndex for Buffer {}

impl Shape for Buffer {
    #[inline]
    fn shape(&self) -> &[Dim; 4] {
        self.shape.shape()
    }
}