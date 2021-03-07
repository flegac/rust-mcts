use std::rc::Rc;

use screen::dimension::Dimension;
use screen::layout::hlayout::HLayout;
use screen::layout::vlayout::VLayout;
use screen::screen::Screen;
use screen::layout::str_layout::StrLayout;

pub type LayoutRc = Rc<dyn Layout>;

pub trait Layout
where
    Self: Dimension,
{
    fn to_screen(&self, x: usize, y: usize, target: &mut Screen);

    //TODO: return &Screen (do not force to create new buffer)
    fn as_screen(&self) -> Screen {
        let mut scr = Screen::new(self.width(), self.height());
        self.to_screen(0, 0, &mut scr);
        scr
    }

    fn to_screen_str(&self) -> String {
        self.as_screen().to_string()
    }

    fn show(&self) {
        self.as_screen().show();
    }
}

pub struct L {}

impl L {
    pub fn str(data: &str) -> Rc<StrLayout> {
        Rc::new(StrLayout::new(data))
    }
    pub fn vert(data: Vec<LayoutRc>) -> LayoutRc {
        Rc::new(VLayout::new(data))
    }
    pub fn hori(data: Vec<LayoutRc>) -> LayoutRc {
        Rc::new(HLayout::new(data))
    }
}
