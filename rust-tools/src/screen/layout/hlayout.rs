use std::iter::FromIterator;

use screen::dimension::Dimension;
use screen::layout::layout::Layout2;
use screen::screen::Screen;

pub struct HLayout<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: Dimension> HLayout<T> {
    pub fn new(data: Vec<T>) -> HLayout<T> {
        let width = data.iter().fold(0, |a, l| a + l.width());
        let height = data.iter().fold(0, |a, l| a.max(l.height()));
        Self { data, width, height }
    }
}

impl<T: Layout2> Dimension for HLayout<T> {
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

impl<T: Layout2> Layout2 for HLayout<T> {
    fn to_screen(&self, x: usize, y: usize, target: &mut Screen) {
        let mut pad = 0;
        let max_height = self.height();

        for l in self.data.iter() {
            let tab = (max_height - l.height()) / 2;
            l.to_screen(x + pad, y + tab, target);
            pad += l.width();
        }
    }
}