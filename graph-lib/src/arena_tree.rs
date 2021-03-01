use std::fmt;
use std::fmt::Display;

use bit_set::BitSet;

type NodeId = usize;
type Label = usize;

type Edge = (Label, NodeId);

struct Node<V> {
    id: NodeId,
    pub value: V,
    parent: Option<Edge>,
    pub children: BitSet,
}

impl<V> Node<V> {
    pub fn new(id: NodeId, value: V) -> Node<V> {
        Self {
            id,
            value,
            parent: None,
            children: BitSet::new(),
        }
    }
}

impl<V> Display for Node<V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

struct TreeArena<V> {
    arena: Vec<Node<V>>,
}

impl<V> TreeArena<V> {
    pub fn new() -> TreeArena<V> {
        TreeArena { arena: vec![] }
    }

    pub fn add_child(&mut self, node: NodeId, label: Label, value: V) -> NodeId {
        let child = self.spawn_node(value);
        self.arena[child].parent = Some((label, node));
        self.arena[node].children.insert(child);
        child
    }

    pub fn get(&self, node_id: NodeId) -> &Node<V> {
        &self.arena[node_id]
    }
    pub fn get_mut(&mut self, node_id: NodeId) -> &mut Node<V> {
        &mut self.arena[node_id]
    }

    pub fn spawn_node(&mut self, value: V) -> NodeId {
        let node_id = self.arena.len();
        let node = Node::new(node_id, value);
        self.arena.push(node);
        node_id
    }
}

#[test]
fn test_creation() {
    let mut arena = TreeArena::new();
    let tree = arena.spawn_node(0);
    let child = arena.add_child(tree, 1, 1);

    println!("{}", arena.get(child));
}
