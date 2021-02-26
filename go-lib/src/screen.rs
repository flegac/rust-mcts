use std::fmt::{Display, Formatter};
use std::fmt;

use itertools::Itertools;

pub struct Screen {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<char>, //FIXME: make private ... fast !
}

//TODO: smart indexes (usize / i32 seamlessly)
impl Screen {
    pub fn border(screen: &Screen) -> Screen {
        let mut res = Self::grow(screen, 1);
        for x in 0..res.width as i32 {
            res.put(x, 0, '-');
            res.put(x, -1, '-');
        }
        for y in 0..res.height as i32 {
            res.put(0, y, '|');
            res.put(-1, y, '|');
        }
        res.put(0, 0, '+');
        res.put(0, -1, '+');
        res.put(-1, 0, '+');
        res.put(-1, -1, '+');
        res
    }

    pub fn grow(screen: &Screen, border: usize) -> Screen {
        let mut res = Self::new(screen.width + 2 * border, screen.height + 2 * border);
        res.draw(border as i32, border as i32, screen);
        res
    }

    pub fn new(width: usize, height: usize) -> Self {
        Self::fill(' ', width, height)
    }

    pub fn fill(value: char, width: usize, height: usize) -> Screen {
        Self { width, height, buffer: vec![value; width * height] }
    }

    pub fn put(&mut self, x: i32, y: i32, value: char) {
        let i = self.at(x, y);
        self.buffer[i] = value;
    }


    pub fn put_slice(&mut self, x: i32, y: i32, src: &[char]) {
        let dst = self.read_mut(y, x, src.len());
        dst.clone_from_slice(src);
    }

    pub fn put_str(&mut self, x: i32, y: i32, src: &String) {
        for (i, c) in src.chars().enumerate() {
            let pos = self.at(x + i as i32, y);
            self.buffer[pos] = c;
        }
    }

    pub fn draw(&mut self, x: i32, y: i32, other: &Screen) {
        for i in 0..other.height as i32 {
            let src = other.read(i as i32, 0, other.width);
            self.put_slice(x, y + i, src);
        }
    }

    fn read(&self, i: i32, j: i32, size: usize) -> &[char] {
        let start = self.at(j, i);
        let end = start + size;
        &self.buffer[start..end]
    }
    fn read_mut(&mut self, i: i32, j: i32, size: usize) -> &mut [char] {
        let start = self.at(j, i);
        let end = start + size;
        &mut self.buffer[start..end]
    }

    pub fn at(&self, x: i32, y: i32) -> usize {
        let w = self.width as i32;
        let h = self.height as i32;
        let x = (x + w) % w;
        let y = (y + h) % h;
        (x + y * w) as usize
    }
}

impl Display for Screen {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut res = String::new();

        for i in 0..self.height as i32 {
            let line: String = self.read(i, 0, self.width)
                .iter()
                .map(|c| format!("{} ", c))
                .collect();
            res.push_str(&format!("{}\n", line));
        }
        write!(f, "{}", res)
    }
}
