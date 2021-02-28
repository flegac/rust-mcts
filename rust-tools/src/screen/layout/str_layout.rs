use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use itertools::Itertools;

use screen::dimension::Dimension;
use screen::drawer::Drawer;
use screen::layout::layout::Layout;
use screen::screen::Screen;

pub type StrPtr = Rc<StrPtr2>;

pub struct StrPtr2 {
    value: RefCell<String>
}

impl StrPtr2 {
    pub fn new(value: &str) -> StrPtr2 {
        StrPtr2 { value: RefCell::new(String::from(value)) }
    }

    pub fn get(&self) -> String {
        let res = self.value.borrow().clone();
        res
    }

    pub fn update(&self, value: &str) {
        self.value.replace(String::from(value));
    }
}

pub struct StrLayout {
    data: StrPtr,
}

impl StrLayout {
    pub fn new(data: &StrPtr) -> StrLayout {
        StrLayout { data: data.clone() }
    }
}

impl Dimension for StrLayout {
    fn width(&self) -> usize {
        self.data.get().lines()
            .map(|l| l.len())
            .fold(0, |a, b| a.max(b))
    }

    fn height(&self) -> usize {
        self.data.get().lines().count()
    }
}

impl Layout for StrLayout {
    fn to_screen(&self, x: usize, y: usize, target: &mut Screen) {
        for (i, l) in self.data.get().lines().enumerate() {
            target.put_str(target.at(x, y + i), l);
        }
    }
}


#[test]
fn test_str2() {
    let mut rc = StrLayout::ptr("coucou");
    let l = StrLayout::new(&rc);

    l.show();
    rc.update("fdsq");
    l.show();
}