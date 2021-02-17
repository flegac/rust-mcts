pub mod node;
pub mod tree;

#[cfg(test)]
mod tests {
    use crate::tree::Tree;

    #[test]
    fn test_it() {
        let root = Tree::new(1);

        root.add_child(&Tree::new(10));
        root.add_child(&Tree::new(11));
        root.add_child(&Tree::new(12));
        println!("{}", &root);
        root.get_child(1).map(|c| {
            c.add_child(&Tree::new(110));
            c.add_child(&Tree::new(111));
            c.add_child(&Tree::new(112));
        });
        println!("{}", &root);
        root.remove(2);
        println!("{}", &root);
        root.set_child(0, &root.get_child(1).unwrap());
        println!("{}", &root);

        let c = root.get_child(0).unwrap().get_child(0).unwrap();


        println!("parents({}) :", c);
        for x in c.parents() {
            println!("- {}", x);
        }
    }
}