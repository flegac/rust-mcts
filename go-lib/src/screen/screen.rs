use std::fmt::{Display, Formatter};
use std::fmt;


use screen::dimension::{Cursor, Dimension, ScreenIndex};
use screen::drawer::Drawer;

pub struct Screen {
    cursor: usize,
    width: usize,
    height: usize,
    pub(crate) buffer: Vec<char>,
}

impl Dimension for Screen {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }
}

impl Cursor for Screen {
    fn offset(&self) -> usize {
        self.cursor
    }

    fn move_to(&mut self, offset: usize) {
        self.cursor = offset;
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
}

impl Screen {
    pub fn new(width: usize, height: usize) -> Self {
        Self::fill(' ', width, height)
    }

    pub fn fill(value: char, width: usize, height: usize) -> Screen {
        Screen { cursor: 0, width, height, buffer: vec![value; width * height] }
    }

    pub fn grow(&self, border: usize) -> Screen {
        let mut res = Self::new(self.width + 2 * border, self.height + 2 * border);
        res.draw_at(res.index(border, border), self);
        res
    }

    pub fn border(&self) -> Screen {
        let mut res = self.grow(1);
        for x in 0..res.width() {
            [0, -1].iter().for_each(|&y| {
                res.put(res.index(x as i32, y), '-');
            });
        }
        for y in 0..res.height() {
            [0, -1].iter().for_each(|&x| {
                res.put(res.index(x, y as i32), '|');
            });
        }
        for &x in [0, -1].iter() {
            for &y in [0, -1].iter() {
                res.put(res.index(x, y), '+');
            }
        }
        res
    }
}

impl Display for Screen {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut res = String::new();

        for i in 0..self.height {
            let line: String = self.read(self.index(0, i), self.width)
                .iter()
                .map(|c| format!("{} ", c))
                .collect();
            res.push_str(&format!("{}\n", line));
        }
        write!(f, "{}", res)
    }
}
