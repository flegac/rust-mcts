use tensor_lib::tensor::Tensor;
use tensor_lib::traits::view::View;

use crate::framework::metric::Metric;

pub struct MSE {}

impl MSE {
    pub fn score_vec(vec: &Vec<f32>) -> f32 {
        let mut score = 0_f32;
        for &x in vec.iter() {
            score += x * x;
        }
        score.sqrt()
    }
}

impl Metric<Tensor> for MSE {
    fn score(lhs: &Tensor, rhs: &Tensor) -> f32 {
        let mut score = 0_f32;
        for i in 0..lhs.shape().len() {
            let x = lhs.get(i) - rhs.get(i);
            score += x * x;
        }
        score.sqrt()
    }
}


#[cfg(test)]
mod tests {
    use tensor_lib::structs::shape4::Shape4;
    use tensor_lib::tensor::Tensor;

    use crate::framework::metric::Metric;
    use crate::framework::metrics::mse::MSE;

    #[test]
    fn test_mse() {
        let shape = Shape4::vec1(2);
        let x1 = Tensor::new(shape.clone(), 0 as f32);
        for i in 0..4 {
            for j in 0..5 {
                let mut x2 = Tensor::new(shape.clone(), j as f32);
                x2.insert(0, i as f32);
                let score1 = MSE::score(&x1, &x2);
                let score2 = MSE::score(&x2, &x1);
                println!("{} : {} {}", score1, x1, x2);
                assert_eq!(score1, score2)
            }
        }
    }
}
