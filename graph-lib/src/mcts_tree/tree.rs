pub trait Tree<T> {
    fn new_node(&mut self, size: usize) -> T;
    fn node_count(&self) -> usize;
    fn node_size(&self, node: &T) -> usize;
    fn display(&self, node: &T);

    fn set_child(&mut self, i: usize, parent: &T, child: &T);
}
