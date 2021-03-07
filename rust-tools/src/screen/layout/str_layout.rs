use screen::dimension::Dimension;
use screen::drawer::Drawer;
use screen::layout::layout::Layout;
use screen::screen::Screen;

pub struct StrLayout {
    data: String,
}

impl StrLayout {
    pub fn new(data: &str) -> StrLayout {
        StrLayout { data: data.to_string() }
    }
}

impl Dimension for StrLayout {
    fn width(&self) -> usize {
        self.data
            .lines()
            .map(|l| l.len())
            .fold(0, |a, b| a.max(b))
    }

    fn height(&self) -> usize {
        self.data.lines().count()
    }
}

impl Layout for StrLayout {
    fn to_screen(&self, x: usize, y: usize, target: &mut Screen) {
        for (i, l) in self.data.lines().enumerate() {
            target.put_str(target.at(x, y + i), l);
        }
    }
}
