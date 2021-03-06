pub trait Mutation<T> {
    fn mutate(&self, adn: &T) -> T;
}
