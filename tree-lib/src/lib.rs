pub mod node;
pub mod tree;
pub mod safe_tree;

#[cfg(test)]
mod tests {
    use std::borrow::BorrowMut;
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::sync::Arc;

    use rpool::{Pool, Poolable, PoolScaleMode};

    use crate::node::N;
    use crate::safe_tree::SafeTree;
    use crate::tree::Tree;

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
        let n0 = Rc::new(x0);
        let n1 = Rc::new(x1);

        n0.children.borrow_mut().push(Rc::clone(&n0));

        println!("{:?}", &n0);
        println!("{:?}", &n1);
    }

    #[test]
    fn test_it() {
        let root = SafeTree::new(1);

        root.add_child(&SafeTree::new(10));
        root.add_child(&SafeTree::new(11));
        root.add_child(&SafeTree::new(12));
        println!("{}", &root);
        root.get_child(1).map(|c| {
            c.add_child(&SafeTree::new(110));
            c.add_child(&SafeTree::new(111));
            c.add_child(&SafeTree::new(112));
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


    #[derive(Debug)]
    struct TestContext {
        test: &'static str,
    }

    #[derive(Debug)]
    struct TestItem {
        test: String,
    }

    impl Poolable<TestContext> for TestItem {
        fn new(context: &TestContext) -> TestItem {
            TestItem {
                test: format!("{}_{}", context.test, "testing item"),
            }
        }

        fn reset(&mut self) -> bool {
            self.borrow_mut().test.clear();
            self.borrow_mut().test.push_str("fds");
            return true;
        }
    }

    #[test]
    fn test_get() {
        let pool: Arc<Pool<TestContext, TestItem>> = Pool::new(
            PoolScaleMode::Static { count: 6 },
            TestContext { test: "testing context" },
        );
        let mut x = vec![];
        for _ in 0..5 {
            let item = pool.get().expect("oups");
            println!("{:?}", item);
            x.push(item);
        }
        for _ in 0..10 {
            let item = pool.get().expect("oups");
            println!("{:?}", item);
        }
    }
}