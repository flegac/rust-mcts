use bit_set::BitSet;

use crate::topology::{Topology, Vert};

pub trait Flood {
    fn flood<Topo, Graph>(&mut self, graph: &Graph, v: usize, topology: &Topo) -> BitSet
    where
        Topo: Fn(Vert) -> bool,
        Graph: Topology;

    fn flood_check<Topo, Stop, Graph>(
        &mut self,
        graph: &Graph,
        v: usize,
        topology: &Topo,
        stop_condition: &Stop,
    ) -> BitSet
    where
        Topo: Fn(Vert) -> bool,
        Stop: Fn(&BitSet) -> bool,
        Graph: Topology;
}
