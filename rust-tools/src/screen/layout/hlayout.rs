
use screen::dimension::Dimension;
use screen::layout::layout::{Layout, LayoutRc};
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
        Self {
            data,
            width,
            height,
        }
    }
}

impl Dimension for HLayout {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }
}

impl Layout for HLayout {
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
