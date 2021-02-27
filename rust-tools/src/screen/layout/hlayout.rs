use std::iter::FromIterator;

use screen::dimension::Dimension;
use screen::layout::layout::{Layout2, LayoutRc};
use screen::screen::Screen;

pub struct HLayout {
    data: Vec<LayoutRc>,
    width: usize,
    height: usize,
}

impl HLayout {
    pub fn new(data: Vec<LayoutRc>) -> HLayout {
        let width = data.iter().fold(0, |a, l| a + l.width());
        let height = data.iter().fold(0, |a, l| a.max(l.height()));
        Self { data, width, height }
    }
}

impl Dimension for HLayout {
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

impl Layout2 for HLayout {
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