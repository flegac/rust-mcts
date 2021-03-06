use crate::metrics::metric::Metric;
use crate::tensors::shape::Shape;
use crate::tensors::tensor::Tensor;

pub struct MSE {}

impl Metric<Tensor> for MSE {
    fn score(lhs: &Tensor, rhs: &Tensor) -> f32 {
        let mut score = 0_f32;
        for i in 0..lhs.volume().unwrap() {
            let delta = lhs.get(i) - rhs.get(i);
            score += delta * delta;
        }
        score.sqrt()
    }
}


#[cfg(test)]
mod tests {
    use crate::metrics::metric::Metric;
    use crate::metrics::mse::MSE;
    use crate::tensors::shape4::Shape4;
    use crate::tensors::tensor::Tensor;

    #[test]
    fn test_mse() {
        let shape = Shape4::vec1(2);
        let x1 = Tensor::new(shape, 0 as f32);
        for i in 0..4 {
            for j in 0..5 {
                let mut x2 = Tensor::new(shape, j as f32);
                x2.insert(0, i as f32);
                let score1 = MSE::score(&x1, &x2);
                let score2 = MSE::score(&x2, &x1);
                println!("{} : {} {}", score1, x1, x2);
                assert_eq!(score1, score2)
            }
        }
    }
}
