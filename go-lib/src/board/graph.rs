use bit_set::BitSet;

pub type Vert = usize;

pub trait Graph {
    fn vertices(&self) -> &BitSet;
    fn edges(&self, v: Vert) -> &BitSet;
    fn flood<F>(&self, v: Vert, test: &F) -> BitSet
        where F: Fn(Vert) -> bool;
}
