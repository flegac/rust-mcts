use itertools::Itertools;

use screen::dimension::Dimension;

pub trait Drawer: Dimension {
    fn put(&mut self, offset: usize, value: char);
    fn read(&self, offset: usize, size: usize) -> &[char];
    fn read_mut(&mut self, offset: usize, size: usize) -> &mut [char];


    fn get(&self, offset: usize) -> &char {
        self.read(offset, 1).iter().next().unwrap()
    }

    fn put_slice(&mut self, offset: usize, src: &[char]) {
        if !src.is_empty() {
            let size = src.len();
            let dst = self.read_mut(offset, size);
            dst.clone_from_slice(src);
        }
    }

    fn put_str(&mut self, offset: usize, src: &str) {
        let vec = src.chars().collect_vec();
        let slice = vec.as_slice();
        self.put_slice(offset, slice);
    }


    fn draw_at(&mut self, offset: usize, other: &Self) {
        let (x, y) = self.xy(offset);
        for i in 0..other.height() {
            if y + i >= self.height() {
                break;
            }
            let k = self.at(x, y + i);

            let src = other.read(other.at(0, i), other.width());
            self.put_slice(k, src);
        }
    }
}
