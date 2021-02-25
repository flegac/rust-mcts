pub mod node;
pub mod safe_tree;
pub mod topology;
pub mod flood;
pub mod graph;

#[cfg(test)]
mod tests {
    use rpool::{Pool, Poolable, PoolScaleMode};
    use std::borrow::BorrowMut;
    use std::sync::Arc;

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