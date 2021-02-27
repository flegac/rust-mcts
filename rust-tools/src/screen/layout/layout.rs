use std::ops::Deref;

use screen::dimension::{Dimension, ScreenIndex};
use screen::drawer::Drawer;
use screen::layout::hlayout::HLayout;
use screen::layout::str_layout::StrLayout;
use screen::layout::vlayout::VLayout;
use screen::screen::Screen;

pub trait Layout where Self: Dimension, Self: Sized {
    fn to_screen(&self, offset: usize, target: &mut Screen);

    fn show(&self) {
        let mut scr = Screen::new(self.width(), self.height());
        self.to_screen(0, &mut scr);
        scr.show()
    }
}

pub struct L {}

impl L {
    pub fn str(data: &str) -> StrLayout {
        StrLayout::new(data)
    }

    pub fn vert<T>(data: Vec<T>) -> VLayout<T> {
        VLayout::new(data)
    }
    pub fn hori<T>(data: Vec<T>) -> HLayout<T> {
        HLayout::new(data)
    }
}

#[test]
fn test() {
    let layout = L::hori(vec![
        L::vert(vec![
            L::str("coucou"),
            L::str("coucou"),
            L::str("coucou"),
        ]),
        L::vert(vec![
            L::str("gfds"),
            L::str("gfds"),
            L::str("gfds"),
        ]),
        L::vert(vec![
            L::str("gfds"),
            L::str("gfds"),
            L::str("gfds"),
        ])
    ]);


    let mut scr = Screen::new(40, 15);

    layout.to_screen(0, &mut scr);

    scr.show();
}