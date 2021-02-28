use std::fmt::{Display, Formatter};
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

use screen::dimension::Dimension;
use screen::drawer::Drawer;
use screen::layout::hlayout::HLayout;
use screen::layout::str_layout::StrLayout;
use screen::layout::vlayout::VLayout;
use screen::screen::Screen;

pub type LayoutRc = Rc<Layout>;

pub trait Layout where Self: Dimension {
    fn to_screen(&self, x: usize, y: usize, target: &mut Screen);
    fn as_screen(&self) -> Screen {
        let mut scr = Screen::new(self.width(), self.height());
        self.to_screen(0, 0, &mut scr);
        scr
    }

    fn to_string(&self) -> String {
        self.as_screen().to_string()
    }

    fn show(&self) {
        self.as_screen().show();
    }
}

pub struct L {}

impl L {
    pub fn str(data: &str) -> LayoutRc {
        Rc::new(StrLayout::new(data))
    }
    pub fn vert(data: Vec<LayoutRc>) -> LayoutRc {
        Rc::new(VLayout::new(data))
    }
    pub fn hori(data: Vec<LayoutRc>) -> LayoutRc {
        Rc::new(HLayout::new(data))
    }
}

#[test]
fn test() {
    let x = L::str("|--7--|");
    let y = L::str("#-5-#");

    let hori1 = L::hori(vec![
        x.clone(), x.clone(), x.clone(), y.clone(), y.clone()
    ]);
    let hori2 = L::hori(vec![
        y.clone(), x.clone(), x.clone(), y.clone()
    ]);

    let vert = L::vert(vec![
        hori1.clone(),
        hori2.clone(),
        hori2.clone(),
        hori1.clone(),
        hori2.clone(),
        hori2.clone()
    ]);
    vert.show();
}