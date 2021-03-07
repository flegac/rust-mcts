use tensor_lib::structs::offset4::Offset4;
use tensor_lib::tensor::Tensor;
use tensor_lib::traits::view::View;

use crate::framework::model::Model;

pub struct Relu {}

impl Relu {
    pub fn new() -> Self {
        Relu {}
    }
}


impl Model<Tensor, Tensor> for Relu {
    fn predict(&self, input: &Tensor, output: &mut Tensor) {
        assert_eq!(input.shape().x(), output.shape().x());
        assert_eq!(input.shape().y(), output.shape().y());
        assert_eq!(input.shape().z(), output.shape().z());
        assert_eq!(output.shape().t(), output.shape().t());

        for z in 0..output.shape().z().unwrap() {
            for x in 0..output.shape().x().unwrap() {
                for y in 0..output.shape().y().unwrap() {
                    let offset = Offset4(x, y, z, 0);
                    let offset = output.shape().index(&offset);
                    let res = input.get(offset).max(0.0);
                    output.insert(offset, res);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use tensor_lib::structs::shape4::Shape4;
    use tensor_lib::tensor::Tensor;
    use tensor_lib::traits::view::View;

    use crate::conv2::Conv2;
    use crate::framework::model::Model;

    #[test]
    fn test_conv2() {
        let input = Tensor::new(Shape4::vec3(10, 10, 4), 1_f32);
        let conv = Conv2::new(5, input.shape().z().unwrap(), 3);


        let mut output = conv.output_tensor(input.shape());

        conv.predict(&input, &mut output);

        println!("{:?}", output)
    }
}
