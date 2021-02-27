use itertools::Itertools;

use screen::dimension::{Cursor};

pub trait Drawer where Self: Cursor {
    fn put(&mut self, offset: usize, value: char);
    fn read(&self, offset: usize, size: usize) -> &[char];
    fn read_mut(&mut self, offset: usize, size: usize) -> &mut [char];

    fn put_slice(&mut self, offset: usize, src: &[char]) {
        let dst = self.read_mut(offset, src.len());
        dst.clone_from_slice(src);
    }

    fn put_str(&mut self, offset: usize, src: &String) {
        let vec = src.chars().collect_vec();
        let slice = vec.as_slice();
        self.put_slice(offset, slice);
        // for (i, c) in src.chars().enumerate() {
        //     let pos = self.at(x + i as i32, y);
        //     self.buffer[pos] = c;
        // }
    }


    fn draw_at(&mut self, offset: usize, other: &Self) {
        let (x, y) = self.xy(offset);
        for i in 0..other.height() {
            let src = other.read(other.at(0, i), other.width());
            let k = self.at(x, y + i);
            self.put_slice(k, src);
        }
    }

    fn draw(&mut self, other: &Self) {
        self.draw_at(self.offset(), other)
    }
}
