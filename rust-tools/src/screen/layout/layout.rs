use std::ops::Deref;

use screen::dimension::{Dimension, ScreenIndex};
use screen::drawer::Drawer;
use screen::layout::hlayout::HLayout;
use screen::layout::str_layout::StrLayout;
use screen::layout::vlayout::VLayout;
use screen::screen::Screen;

pub trait Layout2 where Self: Dimension, Self: Sized {
    fn to_screen(&self, x:usize, y:usize, target: &mut Screen);

    fn as_screen(&self) -> Screen {
        let mut scr = Screen::new(self.width(), self.height());
        self.to_screen(0,0, &mut scr);
        scr
    }

    fn show(&self) {
        self.as_screen().show();
    }
}

pub struct L {}

impl L {
    pub fn str(data: &str) -> StrLayout {
        StrLayout::new(data)
    }

    pub fn vert<T: Layout2>(data: Vec<T>) -> VLayout<T> {
        VLayout::new(data)
    }
    pub fn hori<T: Layout2>(data: Vec<T>) -> HLayout<T> {
        HLayout::new(data)
    }
}

#[test]
fn test() {
    let base = L::str("coucou");
    let hori = L::hori(vec![
        L::str("coucou"),L::str("coucou"),L::str("coucou")
    ]);

    let vert = L::vert(vec![
        L::str("coucou"),L::str("123456789"),L::str("ucou")
    ]);

    assert_eq!(hori.width(), base.width()*3);
    assert_eq!(hori.height(), 1);
    assert_eq!(vert.width(), 9);
    assert_eq!(vert.height(), 3);

    vert.show();
    hori.show();
}