use std::iter::FromIterator;

use bit_set::BitSet;

pub type Vert = usize;

pub trait Topology {
    fn vertices(&self) -> &BitSet;
    fn edges(&self, v: Vert) -> &BitSet;
    fn apply<F: Fn(Vert) -> ()>(&self, op: F) {
        for v in self.vertices() {
            op(v);
        }
    }
    fn vertex_number(&self) -> usize {
        return self.vertices().len();
    }
    fn edges_count(&self, v: Vert) -> usize {
        return self.edges(v).len();
    }
}

pub struct SubGraph {
    vertices: BitSet,
    edges: Vec<BitSet>,
}

impl SubGraph {
    pub fn from<T: Topology, F: Fn(Vert) -> bool>(graph: &T, test: &F) -> SubGraph {
        let mut res = SubGraph {
            vertices: BitSet::from_iter(graph.vertices().iter().filter(|&x| test(x))),
            edges: vec![BitSet::new(); graph.vertex_number()],
        };
        for v in res.vertices.iter() {
            res.edges[v] = BitSet::from_iter(graph.edges(v).iter().filter(|&x| test(x)));
        }

        res
    }
}

impl Topology for SubGraph {
    fn vertices(&self) -> &BitSet {
        &self.vertices
    }
    fn edges(&self, v: usize) -> &BitSet {
        &self.edges[v]
    }
}
