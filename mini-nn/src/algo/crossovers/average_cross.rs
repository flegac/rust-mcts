use rand_distr::{Distribution, Normal, Uniform};

use tensor_lib::tensor::Tensor;
use tensor_lib::traits::view::View;

use crate::algo::crossover::CrossOver;

pub struct AverageCross {
    power: f32
}


impl CrossOver<Tensor> for AverageCross {
    fn cross(&self, m1: &mut Tensor, m2: &mut Tensor) {
        let mut rng = rand::thread_rng();

        let size = m1.shape().len();
        let a = Uniform::new(0, size / 2).sample(&mut rng);
        let b = Uniform::new(a, size).sample(&mut rng);

        for i in 0..size {
            if a < i || i < b {
                m1.insert(i, m1.get(i) + self.power * m2.get(i));
                m2.insert(i, self.power * m1.get(i) + m2.get(i));
            }
        }
    }
}