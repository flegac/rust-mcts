pub trait Policy<A> {
    fn select(&self, items: &[A]) -> A;
}
