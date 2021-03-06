use rand::distributions::Uniform;
use rand_distr::{Distribution, Normal};

use crate::algo::mutation::Mutation;
use tensor_lib::tensor::Tensor;
use tensor_lib::traits::view::View;

pub struct MulMut {
    power: f32
}

impl MulMut {
    pub fn new(power: f32) -> Self {
        MulMut { power }
    }
}

impl Mutation<Tensor> for MulMut {
    fn mutate(&self, m: &mut Tensor) {
        let mut rng = rand::thread_rng();

        let normal = Normal::new(0.0, 1.0).unwrap();
        let offset = Uniform::new(0, m.shape().len()).sample(&mut rng);
        let r = normal.sample(&mut rng);
        let x = m.get(offset) * r * self.power;
        m.insert(offset, x);
    }
}
