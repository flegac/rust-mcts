pub mod node;
pub mod tree;

#[cfg(test)]
mod tests {
    use crate::tree::Tree;
    use crate::node::N;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_it2() {
        let x0 = N {
            data: RefCell::new(0),
            children: RefCell::new(vec![]),
        };
        let x1 = N {
            data: RefCell::new(1),
            children: RefCell::new(vec![]),
        };
        let  n0 = Rc::new(x0);
        let  n1 = Rc::new(x1);

        n0.children.borrow_mut().push(Rc::clone(&n0));

        println!("{:?}", &n0);
        println!("{:?}", &n1);
    }
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