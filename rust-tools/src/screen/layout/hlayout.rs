use std::iter::FromIterator;

use screen::dimension::Dimension;
use screen::screen::Screen;
use screen::layout::layout::Layout;

pub struct HLayout<T> {
    data: Vec<T>,
}

impl<T> HLayout<T> {
    pub fn new(data: Vec<T>) -> HLayout<T> {
        HLayout { data }
    }
}

impl<T: Layout> Dimension for HLayout<T> {
    fn width(&self) -> usize {
        self.data.iter().fold(0, |a, l| a.max(l.width()))
    }

    fn height(&self) -> usize {
        self.data.iter().fold(0, |a, l| a + l.height())
    }

    fn transpose(&mut self) {
        unimplemented!()
    }

    fn is_mirror(&self) -> bool {
        false
    }
}

impl<T: Layout> Layout for HLayout<T> {
    fn to_screen(&self, offset: usize, target: &mut Screen) {
        let (x, y) = target.xy(offset);
        let mut pad = 0;
        for l in self.data.iter() {
            l.to_screen(target.at(x + pad, y), target);
            pad += l.width();
        }
    }
}