use std::borrow::Borrow;
use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::fmt;
use std::rc::Rc;

use rand_distr::{Distribution, Normal};

use crate::structs::offset4::Offset4;
use crate::structs::shape4::Shape4;
use crate::structs::view4::View4;
use crate::traits::view::View;

#[derive(Debug)]
pub struct Tensor {
    buffer: Vec<f32>,
    view: View4,
}

impl Tensor {
    pub fn normal(shape: Shape4, mean: f32, std_dev: f32) -> Tensor {
        Self::from_distrib(shape, Normal::new(mean, std_dev).unwrap())
    }

    pub fn from_distrib<D: Distribution<f32>>(shape: Shape4, dist: D) -> Self {
        let mut rng = rand::thread_rng();
        let mut buffer = vec![0_f32; shape.len()];
        for i in 0..buffer.len() {
            buffer[i] = dist.sample(&mut rng);
        }
        Self::from_buffer(buffer, View4::new(shape))
    }

    pub fn new(shape: Shape4, value: f32) -> Self {
        Self::from_buffer(
            vec![value; shape.len()],
            View4::new(shape))
    }

    pub fn from_buffer(buffer: Vec<f32>, view: View4) -> Self {
        assert_eq!(buffer.len(), view.shape().len());
        Tensor {
            buffer,
            view,
        }
    }

    pub fn view(&self, offset: Offset4, shape: Shape4) -> Tensor {
        Tensor {
            buffer: self.buffer.clone(),
            view: View4 {
                offset,
                shape,
            },
        }
    }

    pub fn get_at(&self, offset: Offset4) -> f32 {
        log::trace!("Offset4.index_from");

        self.get(self.shape().index(&offset))
    }

    pub fn insert_at(&mut self, offset: Offset4, value: f32) {
        self.insert(self.shape().index(&offset), value)
    }

    pub fn get(&self, offset: usize) -> f32 {
        self.buffer[offset]
    }

    pub fn insert(&mut self, offset: usize, value: f32) {
        self.buffer[offset] = value;
    }

    pub fn copy_from(&mut self, other: &Tensor) {
        self.buffer.as_mut_slice().copy_from_slice(
            other.buffer.as_slice()
        )
    }

    pub fn deep_clone(&self) -> Tensor {
        Tensor::from_buffer(
            self.buffer.clone(),
            self.view.clone(),
        )
    }
}

impl View for Tensor {
    fn offset(&self) -> &Offset4 {
        self.view.offset()
    }

    fn shape(&self) -> &Shape4 {
        &self.view.shape
    }
}


impl Display for Tensor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.buffer)
    }
}

impl Clone for Tensor {
    fn clone(&self) -> Self {
        self.deep_clone()
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use rust_tools::bench::Bench;

    use crate::structs::shape4::Shape4;
    use crate::tensor::Tensor;

    #[test]
    fn test_tensor() {
        let shape = Shape4::vec4(32, 32, 128, 1);
        let mut x = Tensor::normal(shape.clone(), 0.0, 1.0);
        let y = Tensor::normal(shape, 0.0, 1.0);

        let mut bench = Bench::new("Tensor Mut");
        while bench.for_duration(Duration::from_secs(5)) {
            x *= y.clone();
        }
        // println!("{:?}", x);
        println!("{}", bench);
    }
}
