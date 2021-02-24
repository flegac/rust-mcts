use std::mem;

use bit_set::BitSet;

use crate::topology::{Topology, Vert};
use crate::flood::Flood;

pub struct Graph {
    visited: BitSet,
    to_visit: BitSet,
    connected: BitSet,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            visited: BitSet::new(),
            to_visit: BitSet::new(),
            connected: BitSet::new(),
        }
    }
}

impl Flood for Graph {
    fn flood<F, G>(mut self, graph: &G, v: usize, topology: &F) -> BitSet
        where
            F: Fn(Vert) -> bool,
            G: Topology {
        let cond = |_: &BitSet| false;
        self.flood_check(graph, v, topology, &cond)
    }

    fn flood_check<F, T, G>(mut self, graph: &G, v: usize, topology: &F, stop_condition: &T) -> BitSet
        where
            F: Fn(Vert) -> bool,
            T: Fn(&BitSet) -> bool,
            G: Topology {
        self.visited.clear();
        self.to_visit.clear();
        self.to_visit.insert(v);
        self.visited.insert(v);

        while !self.to_visit.is_empty() && !stop_condition(&self.visited) {
            self.connected.clear();
            for c in self.to_visit.iter() {
                for cc in graph.edges(c).iter() {
                    if topology(cc) && !self.visited.contains(cc) {
                        self.connected.insert(cc);
                    }
                }
            }
            self.visited.union_with(&self.connected);
            mem::swap(&mut self.to_visit, &mut self.connected);
        }
        self.visited.clone()
    }
}