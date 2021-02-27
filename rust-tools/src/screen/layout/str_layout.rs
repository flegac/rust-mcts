use screen::dimension::Dimension;
use screen::drawer::Drawer;
use screen::layout::layout::Layout2;
use screen::screen::Screen;

pub struct StrLayout {
    data: String
}

impl StrLayout {
    pub fn new(data: &str) -> StrLayout {
        StrLayout {
            data: String::from(data)
        }
    }
}

impl Dimension for StrLayout {
    fn width(&self) -> usize {
        self.data.len()
    }

    fn height(&self) -> usize {
        1
    }

    fn transpose(&mut self) {
        unimplemented!()
    }

    fn is_mirror(&self) -> bool {
        false
    }
}

impl Layout2 for StrLayout {
    fn to_screen(&self, x: usize, y: usize, target: &mut Screen) {
        target.put_str(target.at(x, y), self.data.as_str());
    }
}
