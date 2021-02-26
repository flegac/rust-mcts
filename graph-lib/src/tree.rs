pub trait TheTree<K, V> where Self: Sized {
    fn parent(&self) -> Option<(K, Self)>;
    fn set_child(&self, index: K, value: &Self);
    fn get_child(&self, index: K) -> Option<Self>;
    fn remove(&self, index: K);
}