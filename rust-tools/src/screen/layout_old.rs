use std::ops::Add;

use screen::dimension::{Dimension, ScreenIndex};
use screen::drawer::Drawer;
use screen::screen::Screen;

pub trait Layout {
    fn valign(screens: &[&Screen], pad: usize) -> Screen;
    fn halign(screens: &[&Screen], pad: usize) -> Screen;
}


impl Layout for Screen {
    fn valign(screens: &[&Screen], pad: usize) -> Screen {
        let w = screens.iter()
            .map(|s| s.width())
            .fold(0, usize::max);
        let h = screens.iter()
            .map(|s| s.height() + pad)
            .fold(0, usize::add) - pad;
        let mut scr = Screen::new(w, h);
        let mut offset = 0;
        for s in screens {
            let tab = (scr.width() - s.width()) / 2;
            scr.draw_at(scr.index(tab, offset), s);
            offset += s.height() + pad;
        }
        scr
    }

    fn halign(screens: &[&Screen], pad: usize) -> Screen {
        let w = screens.iter()
            .map(|s| s.width() + pad)
            .fold(0, usize::add) - pad;
        let h = screens.iter()
            .map(|s| s.height())
            .fold(0, usize::max);
        let mut scr = Screen::new(w, h);
        let mut offset = 0;
        for s in screens {
            let tab = (scr.height() - s.height()) / 2;
            scr.draw_at(scr.index(offset, tab), s);
            offset += s.width() + pad;
        }
        scr
    }
}