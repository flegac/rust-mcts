use std::iter::FromIterator;

use screen::dimension::Dimension;
use screen::layout::layout::Layout2;
use screen::screen::Screen;

pub struct VLayout<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: Dimension> VLayout<T> {
    pub fn new(data: Vec<T>) -> VLayout<T> {
        let width = data.iter().fold(0, |a, l| a.max(l.width()));
        let height = data.iter().fold(0, |a, l| a + l.height());
        Self { data, width, height }
    }
}

impl<T: Layout2> Dimension for VLayout<T> {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn transpose(&mut self) {
        unimplemented!()
    }

    fn is_mirror(&self) -> bool {
        false
    }
}

impl<T: Layout2> Layout2 for VLayout<T> {
    fn to_screen(&self, x: usize, y: usize, target: &mut Screen) {
        let mut pad = 0;
        let max_width = self.width();
        for l in self.data.iter() {
            let tab = (max_width - l.width()) / 2;
            l.to_screen(x + tab, y + pad, target);
            pad += l.height();
        }
    }
}