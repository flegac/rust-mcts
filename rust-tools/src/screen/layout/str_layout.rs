use screen::dimension::Dimension;
use screen::layout::layout::Layout;
use screen::screen::Screen;
use screen::drawer::Drawer;

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

impl Layout for StrLayout {
    fn to_screen(&self, offset: usize, target: &mut Screen) {
        target.put_str(offset, self.data.as_str());
    }
}
