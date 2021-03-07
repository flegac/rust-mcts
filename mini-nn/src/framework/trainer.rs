use crate::framework::model::Model;

pub trait Trainer<X, Y, M: Model<X, Y>> {
    fn fit(&self, model: &mut M, x: &Vec<X>, y: &Vec<Y>);
}
