use crate::shape4::Shape4;
use crate::shape::Shape;
use crate::tensor::Tensor;

pub trait Layer {
    fn compute(&self, input: &Tensor, output: &mut Tensor);
}
