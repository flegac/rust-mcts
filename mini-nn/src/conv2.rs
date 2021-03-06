use std::borrow::Borrow;

use crate::buffer::Buffer;
use crate::dim::Dim;
use crate::layer::Layer;
use crate::shape::{Shape, ShapeIndex};
use crate::shape4::{NDIMS, Shape4};
use crate::tensor::Tensor;

struct Conv2 {
    filter: Tensor,
    bias: Tensor,
}

impl Shape for Conv2 {
    #[inline]
    fn shape(&self) -> &[Dim; NDIMS] {
        self.filter.shape()
    }
}

impl Conv2 {
    pub fn new(kernel_size: usize, in_features: usize, out_features: usize) -> Self {
        Conv2 {
            filter: Tensor::from_shape(
                Shape4::vec4(kernel_size, kernel_size, in_features, out_features),
                1_f32,
            ),
            bias: Tensor::from_shape(
                Shape4::vec4(kernel_size, kernel_size, in_features, out_features),
                1_f32,
            ),
        }
    }

    pub fn output_tensor<T: Shape>(&self, input_shape: T) -> Tensor {
        let output_shape = Shape4::vec3(
            input_shape.x().unwrap() - self.x().unwrap() + 1,
            input_shape.y().unwrap() - self.y().unwrap() + 1,
            self.t().unwrap(),
        );
        Tensor::from_shape(output_shape, 0_f32)
    }


    fn compute_at(&self, out_x: usize, out_y: usize, feat_out: usize, input: &Tensor) -> f32 {
        let kw = self.x().unwrap();
        let kh = self.y().unwrap();
        let kd = self.z().unwrap();

        assert!(out_x < input.x().unwrap() - kw + 1);
        assert!(out_y < input.y().unwrap() - kh + 1);
        assert!(feat_out < self.t().unwrap());
        let mut res = 0_f32;
        for feat_in in 0..kd {
            for kx in 0..kw {
                for ky in 0..kh {
                    let kernel_offset = self.filter.index(kx, ky, feat_in, 0);
                    let input_offset = input.index(out_x + kx, out_y + ky, feat_in, 0);
                    let a = self.filter.buffer.as_ref().borrow().data[kernel_offset] as f32;
                    let b = self.bias.buffer.as_ref().borrow().data[kernel_offset] as f32;
                    let k = input.buffer.as_ref().borrow().data[input_offset];
                    res += a * k + b
                }
            }
        }
        res
    }
}

impl Layer for Conv2 {
    fn compute(&self, input: &Tensor, output: &mut Tensor) {
        assert_eq!(input.z().unwrap(), self.z().unwrap());
        assert_eq!(output.z().unwrap(), self.t().unwrap());
        assert_eq!(output.x().unwrap(), input.x().unwrap() - self.x().unwrap() + 1);
        assert_eq!(output.y().unwrap(), input.y().unwrap() - self.y().unwrap() + 1);

        // let input_shape = Shape4::new(self.x(), self.y(), self.z(), Dim::Size(1));
        // let output_shape = Shape4::new(self.x(), self.y(), Dim::Size(1), Dim::Size(1));

        for feat_o in 0..output.z().unwrap() {
            for x in 0..output.x().unwrap() {
                for y in 0..output.y().unwrap() {
                    let res = self.compute_at(x, y, feat_o, input);
                    let offset = output.view.shape.index(x, y, feat_o, 0);
                    output.buffer.as_ref().borrow_mut().data[offset] = res;
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

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;

    use crate::buffer::Buffer;
    use crate::conv2::Conv2;
    use crate::layer::Layer;
    use crate::shape::{Shape, ShapeIndex};
    use crate::shape4::Shape4;
    use crate::tensor::Tensor;

    #[test]
    fn test_conv2() {
        let input_shape = Shape4::vec3(10, 10, 4);
        let input = Tensor::from_shape(input_shape, 1_f32);

        let conv = Conv2::new(5, input_shape.z().unwrap(), 3);


        let mut output = conv.output_tensor(input_shape);

        conv.compute(&input, &mut output);

        println!("{:?}", output)
    }
}
