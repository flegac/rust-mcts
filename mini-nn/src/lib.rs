#[macro_use]
extern crate itertools;

pub mod conv2;
pub mod algo;
pub mod relu;
pub mod framework;


#[cfg(test)]
mod tests {
    use tch::Tensor;

    #[test]
    fn test_tensor() {
        fn main() {
            let t = Tensor::of_slice(&[3, 1, 4, 1, 5]);
            let t = t * 2;
            t.print();
        }
    }
}
