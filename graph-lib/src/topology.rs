use bit_set::BitSet;

pub type Vert = usize;

pub trait Topology {
    fn vertices(&self) -> &BitSet;
    fn edges(&self, v: Vert) -> &BitSet;
}
