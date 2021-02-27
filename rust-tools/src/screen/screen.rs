use std::{fmt, mem};
use std::fmt::{Display, Formatter};

use crate::screen::dimension::{Cursor, Dimension, ScreenIndex};
use crate::screen::drawer::Drawer;

pub struct Screen {
    cursor: usize,
    is_mirror: bool,
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
        Screen {
            is_mirror: false,
            cursor: 0,
            width,
            height,
            buffer: vec![' '; width * height],
        }
    }

    pub fn fill(&mut self, value: char) {
        self.buffer.as_mut_slice().fill(value);
    }

    pub fn sparse(&self) -> Screen {
        let mut res = Self::new(self.width() * 3, self.height());
        for offset in 0..self.buffer.len() {
            res.put(1 + 3 * offset, self.buffer[offset]);
        }
        res
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

    pub fn show(&self) {
        println!("{}", self);
    }
}

impl Dimension for Screen {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn transpose(&mut self) {
        mem::swap(&mut self.width, &mut self.height);
        self.is_mirror = !self.is_mirror;
    }

    fn is_mirror(&self) -> bool {
        self.is_mirror
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

impl Display for Screen {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut res = String::new();

        for i in 0..self.height() {
            for j in 0..self.width() {
                let &x = if self.is_mirror() {
                    self.get(self.index(i, j))
                } else {
                    self.get(self.index(j, i))
                };
                res.push(char::from(x));
            }
            res.push('\n');
        }
        write!(f, "{}", res)
    }
}
