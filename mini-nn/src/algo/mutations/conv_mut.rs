use tensor_lib::tensor::Tensor;

use crate::algo::mutations::mutation::Mutation;
use crate::conv2::Conv2;

pub struct ConvMut<M: Mutation<Tensor>> {
    bias: Option<M>,
    filter: Option<M>,
}

impl<M: Mutation<Tensor>> ConvMut<M> {
    pub fn bias(bias: M) -> Self {
        ConvMut { bias: Some(bias), filter: None }
    }
    pub fn filter(filter: M) -> Self {
        ConvMut { bias: None, filter: Some(filter) }
    }
}


impl<M: Mutation<Tensor>> Mutation<Conv2> for ConvMut<M> {
    fn mutate(&self, adn: &mut Conv2) {
        println!("mutate Conv2");
        match &self.bias {
            Some(mutation) => {
                mutation.mutate(&mut adn.bias)
            }
            _ => {}
        }
        match &self.filter {
            Some(mutation) => {
                mutation.mutate(&mut adn.filter)
            }
            _ => {}
        }
    }
}