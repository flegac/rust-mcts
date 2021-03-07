use itertools::Itertools;

pub trait Model<X, Y> {
    fn predict(&self, x: &X, y: &mut Y);
    fn predict_map(&self, x: &Vec<X>, y: &mut Vec<Y>) {
        for i in 0..x.len() {
            self.predict(&x[i], &mut y[i]);
        }
    }
}
