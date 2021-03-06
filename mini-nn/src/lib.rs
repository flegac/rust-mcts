use buffer::Buffer;
use shape4::Shape4;

use crate::shape::Shape;

mod shape;
mod tensor;
mod conv2;
mod layer;
mod shape4;
mod dim;
mod buffer;
mod algo;


#[cfg(test)]
mod tests {
    use crate::shape4::Shape4;

    #[test]
    fn test_tensor() {}
}
