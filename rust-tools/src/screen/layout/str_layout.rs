use itertools::Itertools;

use screen::dimension::Dimension;
use screen::drawer::Drawer;
use screen::layout::layout::Layout;
use screen::screen::Screen;

pub struct StrLayout {
    data: Vec<String>,
    width: usize,
    height: usize,
}

impl StrLayout {
    pub fn new(data: &str) -> StrLayout {
        let lines = data.lines().map(String::from).collect_vec();
        let width = lines.iter().fold(0, |a, l| a.max(l.len()));
        let height = lines.len();
        StrLayout { data: lines, width, height }
    }
}

impl Dimension for StrLayout {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }
}

impl Layout for StrLayout {
    fn to_screen(&self, x: usize, y: usize, target: &mut Screen) {
        for (i, l) in self.data.iter().enumerate() {
            target.put_str(target.at(x, y + i), l);
        }
    }
}
