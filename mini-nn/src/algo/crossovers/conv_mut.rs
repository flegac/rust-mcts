use crate::conv2::Conv2;
use tensor_lib::tensor::Tensor;
use crate::algo::crossover::CrossOver;

pub struct ConvCross<Cr: CrossOver<Tensor>> {
    pub bias: Option<Cr>,
    pub filter: Option<Cr>,
}

impl<Cr: CrossOver<Tensor>> ConvCross<Cr> {
    pub fn bias(bias: Cr) -> Self {
        ConvCross { bias: Some(bias), filter: None }
    }
    pub fn filter(filter: Cr) -> Self {
        ConvCross { bias: None, filter: Some(filter) }
    }
}


impl<Cr: CrossOver<Tensor>> CrossOver<Conv2> for ConvCross<Cr> {
    fn cross(&self, m1: &mut Conv2, m2: &mut Conv2) {
        match &self.bias {
            Some(mutation) => {
                mutation.cross(&mut m1.bias, &mut m2.bias);
            }
            _ => {}
        }
        match &self.filter {
            Some(mutation) => {
                mutation.cross(&mut m1.filter, &mut m2.filter);
            }
            _ => {}
        }
    }
}