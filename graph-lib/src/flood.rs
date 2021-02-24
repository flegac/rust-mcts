use std::mem;

use bit_set::BitSet;

use crate::topology::{Topology, Vert};

pub trait Flood {
    fn flood<F, G>(&mut self, graph: &G, v: usize, topology: &F) -> BitSet
        where
            F: Fn(Vert) -> bool,
            G: Topology;
    fn flood_check<F, T, G>(&mut self, graph: &G, v: usize, topology: &F, stop_condition: &T) -> BitSet
        where
            F: Fn(Vert) -> bool,
            T: Fn(&BitSet) -> bool,
            G: Topology;
}
