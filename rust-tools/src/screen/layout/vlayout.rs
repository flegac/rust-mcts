use std::iter::FromIterator;

use screen::dimension::Dimension;
use screen::screen::Screen;
use screen::layout::layout::Layout;

pub struct VLayout<T> {
    data: Vec<T>,
}

impl<T> VLayout<T> {
    pub fn new(data: Vec<T>) -> VLayout<T> {
        VLayout { data }
    }
}

impl<T: Layout> Dimension for VLayout<T> {
    fn width(&self) -> usize {
        self.data.iter().fold(0, |a, l| a + l.width())
    }

    fn height(&self) -> usize {
        self.data.iter().fold(0, |a, l| a.max(l.height()))
    }

    fn transpose(&mut self) {
        unimplemented!()
    }

    fn is_mirror(&self) -> bool {
        false
    }
}

impl<T: Layout> Layout for VLayout<T> {
    fn to_screen(&self, offset: usize, target: &mut Screen) {
        let (x, y) = target.xy(offset);
        let mut pad = 0;
        for l in self.data.iter() {
            l.to_screen(target.at(x, y + pad), target);
            pad += l.height();
        }
    }
}

