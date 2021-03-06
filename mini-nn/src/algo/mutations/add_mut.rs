use rand_distr::Distribution;
use rand_distr::Normal;

use crate::algo::mutations::mutation::Mutation;
use crate::tensors::shape::Shape;
use crate::tensors::tensor::Tensor;

pub struct AddMut {
    pub power: f32
}

impl AddMut {
    pub fn new(power: f32) -> Self {
        AddMut { power }
    }
}

impl Mutation<Tensor> for AddMut {
    fn mutate(&self, adn: &mut Tensor) {
        let mut rng = rand::thread_rng();

        let normal = Normal::new(0.0, 1.0).unwrap();
        for i in 0..adn.volume().unwrap() {
            let r = normal.sample(&mut rng);
            if r > 0.0 {
                let x = adn.get(i) + self.power;
                adn.insert(i, x);
            }
        }
    }
}
