use tensor_lib::tensor::Tensor;

use crate::algo::mutation::Mutation;
use crate::conv2::Conv2;

pub struct ConvMut<Mut: Mutation<Tensor>> {
    pub bias: Option<Mut>,
    pub filter: Option<Mut>,
}

impl<Mut: Mutation<Tensor>> ConvMut<Mut> {
    pub fn bias(bias: Mut) -> Self {
        ConvMut { bias: Some(bias), filter: None }
    }
    pub fn filter(filter: Mut) -> Self {
        ConvMut { bias: None, filter: Some(filter) }
    }
}


impl<Mut: Mutation<Tensor>> Mutation<Conv2> for ConvMut<Mut> {
    fn mutate(&self, m: &mut Conv2) {
        match &self.bias {
            Some(mutation) => {
                mutation.mutate(&mut m.bias);
            }
            _ => {}
        }
        match &self.filter {
            Some(mutation) => {
                mutation.mutate(&mut m.filter);
            }
            _ => {}
        }
    }
}