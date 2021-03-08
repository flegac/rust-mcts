use rand_distr::Distribution;
use rand_distr::Normal;

use tensor_lib::tensor::Tensor;
use tensor_lib::traits::view::View;

use crate::algo::mutation::Mutation;
use rand::distributions::Uniform;

pub struct AddMut {
    pub power: f32
}

impl AddMut {
    pub fn new(power: f32) -> Self {
        AddMut { power }
    }
}

impl Mutation<Tensor> for AddMut {
    fn mutate(&self, m: &mut Tensor) {
        let mut rng = rand::thread_rng();
        let normal = Normal::new(0.0, 1.0).unwrap();
        let offset = Uniform::new(0, m.shape().len()).sample(&mut rng);
        let r = normal.sample(&mut rng);
        let x = m.get(offset)  + r * self.power;
        m.insert(offset, x);


        // let normal = Normal::new(0.0, 1.0).unwrap();
        // for i in 0..adn.shape().len() {
        //     let r = normal.sample(&mut rng);
        //     if r > 0.0 {
        //         let x = adn.get(i) + self.power;
        //         adn.insert(i, x);
        //     }
        // }
    }
}
