use std::fmt::{Display, Formatter};
use std::fmt;

use rand_distr::Normal;

use tensor_lib::structs::offset4::Offset4;
use tensor_lib::structs::shape4::Shape4;
use tensor_lib::tensor::Tensor;
use tensor_lib::traits::view::View;

use crate::framework::model::Model;

pub struct Conv2 {
    pub filter: Tensor,
    pub bias: Tensor,
}

impl Conv2 {
    pub fn new(kernel_size: usize, in_features: usize, out_features: usize) -> Self {
        let shape = Shape4::vec4(kernel_size, kernel_size, in_features, out_features);
        Conv2 {
            filter: Tensor::normal(shape.clone(), 0.0, 1.0),
            bias: Tensor::normal(shape.clone(), 0.0, 1.0),
        }
    }


    pub fn output_tensor(&self, input_shape: &Shape4) -> Tensor {
        let output_shape = Shape4::vec3(
            input_shape.x().unwrap() - self.shape().x().unwrap() + 1,
            input_shape.y().unwrap() - self.shape().y().unwrap() + 1,
            self.shape().t().unwrap(),
        );
        Tensor::new(output_shape, 0_f32)
    }


    fn compute_at(&self, out_x: usize, out_y: usize, feat_out: usize, input: &Tensor) -> f32 {
        let kw = self.shape().x().unwrap();
        let kh = self.shape().y().unwrap();
        let kd = self.shape().z().unwrap();

        assert!(out_x < input.shape().x().unwrap() - kw + 1);
        assert!(out_y < input.shape().y().unwrap() - kh + 1);
        assert!(feat_out < self.shape().t().unwrap());
        let mut res = 0_f32;
        for feat_in in 0..kd {
            for kx in 0..kw {
                for ky in 0..kh {
                    let offset = Offset4(kx, ky, feat_in, 0);
                    let kernel_offset = self.filter.shape().index(&offset);
                    let a = self.filter.get(kernel_offset);
                    let b = self.bias.get(kernel_offset);

                    let offset2 = Offset4(out_x + kx, out_y + ky, feat_in, 0);
                    let input_offset = input.shape().index(&offset2);
                    let k = input.get(input_offset);
                    res += a * k + b
                }
            }
        }
        res
    }
}

impl View for Conv2 {
    fn offset(&self) -> &Offset4 {
        self.filter.offset()
    }

    fn shape(&self) -> &Shape4 {
        self.filter.shape()
    }
}

impl Model<Tensor, Tensor> for Conv2 {
    fn predict(&self, input: &Tensor, output: &mut Tensor) {
        assert_eq!(input.shape().z(), self.shape().z());
        assert_eq!(output.shape().z(), self.shape().t());
        assert_eq!(output.shape().x().unwrap(), (input.shape().x() - self.shape().x()).unwrap() + 1);
        assert_eq!(output.shape().y().unwrap(), (input.shape().y() - self.shape().y()).unwrap() + 1);

        // let input_shape = Shape4::new(self.x(), self.y(), self.z(), Dim::Size(1));
        // let output_shape = Shape4::new(self.x(), self.y(), Dim::Size(1), Dim::Size(1));

        for feat_o in 0..output.shape().z().unwrap() {
            for x in 0..output.shape().x().unwrap() {
                for y in 0..output.shape().y().unwrap() {
                    let res = self.compute_at(x, y, feat_o, input);
                    let offset = Offset4(x, y, feat_o, 0);
                    let offset = output.shape().index(&offset);
                    output.insert(offset, res);
                    // let mut in_ = input.view((x, y, 0, 0), input_shape);
                    // let mut out_ = output.view((x, y, feat_o, 0), output_shape);
                    // out_ += input.clone();
                    // out_ *= self.filter.clone();
                    // out_ += self.bias.clone();
                }
            }
        }
    }
}

impl Display for Conv2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "filter: {}\nbias: {}\n", self.filter, self.bias)
    }
}

impl Clone for Conv2 {
    fn clone(&self) -> Self {
        Conv2 {
            bias: self.bias.deep_clone(),
            filter: self.filter.deep_clone(),
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
