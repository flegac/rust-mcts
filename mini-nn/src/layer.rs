use crate::tensors::tensor::Tensor;

pub trait Layer {
    fn compute(&self, input: &Tensor, output: &mut Tensor);
}
