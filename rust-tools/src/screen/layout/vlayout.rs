use std::iter::FromIterator;

use screen::dimension::Dimension;
use screen::layout::layout::{Layout, LayoutRc};
use screen::screen::Screen;


enum LayoutPolicy {
    Vertical,
    Horizontal


}

pub struct VLayout {
    data: Vec<LayoutRc>,
    width: usize,
    height: usize,
}

impl VLayout {
    pub fn new(data: Vec<LayoutRc>) -> VLayout {
        let width = data.iter().fold(0, |a, l| a.max(l.width()));
        let height = data.iter().fold(0, |a, l| a + l.height());
        Self { data, width, height }
    }
}

impl Dimension for VLayout {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }
}

impl Layout for VLayout {
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