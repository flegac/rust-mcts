use std::{fmt, mem};
use std::fmt::{Display, Formatter};

use screen::dimension::{Dim, Dimension};
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
        Screen {
            width,
            height,
            buffer: vec![' '; width * height],
        }
    }

    // pub fn grow(&self, border: usize) -> Screen {
    //     let mut res = Self::new(self.width + 2 * border, self.height + 2 * border);
    //     res.draw_at(res.index(border, border), self);
    //     res
    // }
    //
    // pub fn border(&self) -> Screen {
    //     let mut res = self.grow(1);
    //     for x in 0..res.width() {
    //         [0, -1].iter().for_each(|&y| {
    //             res.put(res.index(x as i32, y), '-');
    //         });
    //     }
    //     for y in 0..res.height() {
    //         [0, -1].iter().for_each(|&x| {
    //             res.put(res.index(x, y as i32), '|');
    //         });
    //     }
    //     for &x in [0, -1].iter() {
    //         for &y in [0, -1].iter() {
    //             res.put(res.index(x, y), '+');
    //         }
    //     }
    //     res
    // }

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
}

impl Drawer for Screen {
    fn put(&mut self, offset: usize, value: char) {
        self.buffer[offset] = value;
    }

    fn get(&self, offset: usize) -> &char {
        &self.buffer[offset]
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
        for i in 0..self.height() {
            for j in 0..self.width() {
                write!(f, "{}", self.get(self.index(j, i)));
            }
            write!(f, "\n");
        }
        write!(f, "", )
    }
}
