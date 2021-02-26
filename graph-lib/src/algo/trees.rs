pub trait Trees<K, V> where Self: Sized {
    fn search_max_child<B: Ord, F>(&self, f: F) -> Option<(K, Self)>
        where F: Fn(&V) -> B;
}
