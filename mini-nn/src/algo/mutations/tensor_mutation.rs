use crate::algo::mutations::mutation::Mutation;
use crate::tensors::tensor::Tensor;

pub trait TensorMutation: Mutation<Tensor> + Sized {}

impl<M: Mutation<Tensor>> TensorMutation for M {}
