pub trait Tree where Self: Sized {
    fn parent(&self) -> Option<Self>;
    fn set_child(&self, index: usize, value: &Self);
    fn remove(&self, index: usize);
    fn get_child(&self, index: usize) -> Option<Self>;
    fn parents(&self) -> Vec<Self>;
    fn add_child(&self, tree: &Self);
}

