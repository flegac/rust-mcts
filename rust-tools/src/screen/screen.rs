use std::fmt;
use std::fmt::{Display, Formatter};

use screen::dimension::Dimension;
use screen::layout::layout::Layout;
use screen::smart_index::SmartIndex;

use crate::screen::drawer::Drawer;

pub struct Screen {
    width: usize,
    height: usize,
    pub(crate) buffer: Vec<char>,
}

impl Screen {
    pub fn from_string(value: &str) -> Screen {
        let mut res = Self::new(value.len(), 1);
        res.put_str(0, value);
        res
    }

    pub fn new(width: usize, height: usize) -> Self {
        log::debug!("SCREEN::NEW {}x{}", width, height);
        Screen {
            width,
            height,
            buffer: vec![' '; width * height],
        }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.buffer.resize_with(width * height, || ' ');
        self.width = width;
        self.height = height;
    }

    pub fn show(&self) {
        println!("{}", self);
    }
}

// impl Layout for Screen {
//     fn to_screen(&self, x: usize, y: usize, target: &mut Screen) {
//         target.draw_at(target.at(x, y), self);
//     }
// }

impl Dimension for Screen {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }
}

impl Drawer for Screen {
    fn put(&mut self, offset: usize, value: char) {
        self.buffer[offset] = value;
    }

    fn read(&self, offset: usize, size: usize) -> &[char] {
        let end = offset + size;
        &self.buffer[offset..end]
    }

    fn read_mut(&mut self, offset: usize, size: usize) -> &mut [char] {
        let end = offset + size;
        &mut self.buffer[offset..end]
    }

    fn get(&self, offset: usize) -> &char {
        &self.buffer[offset]
    }
}

impl Display for Screen {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for i in 0..self.height() {
            for j in 0..self.width() {
                write!(f, "{}", self.get(self.index(j, i)))?;
            }
            write!(f, "\n")?;
        }
        write!(f, "")
    }
}
