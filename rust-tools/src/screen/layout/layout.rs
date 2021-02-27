use std::ops::Deref;

use screen::dimension::{Dimension, ScreenIndex};
use screen::drawer::Drawer;
use screen::layout::hlayout::HLayout;
use screen::layout::str_layout::StrLayout;
use screen::layout::vlayout::VLayout;
use screen::screen::Screen;
use std::rc::Rc;

pub type LayoutRc = Rc<Layout2>;

pub trait Layout2 where Self: Dimension {
    fn to_screen(&self, x: usize, y: usize, target: &mut Screen);

    fn as_screen(&self) -> Screen {
        let mut scr = Screen::new(self.width(), self.height());
        self.to_screen(0, 0, &mut scr);
        scr
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
    // assert_eq!(hori.width(), base.width()*3);
    // assert_eq!(hori.height(), 1);
    // assert_eq!(vert.width(), 9);
    // assert_eq!(vert.height(), 3);

    vert.show();
}