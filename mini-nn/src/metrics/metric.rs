use std::iter::FromIterator;

pub trait Metric<T> {
    fn score(lhs: &T, rhs: &T) -> f32;

    fn score_map(lhs: &T, rhs: &[T]) -> f32 {
        let mut res = 0_f32;
        for i in 0..rhs.len() {
            let delta = Self::score(lhs, &rhs[i]);
            res += delta * delta;
        }
        res.sqrt()
    }

    fn score_zip(lhs: &[T], rhs: &[T]) -> Vec<f32> {
        Vec::from_iter(
            (0..lhs.len())
                .map(|i| Self::score(&lhs[i], &rhs[i]))
        )
    }
}
