use screen::dimension::Dimension;

pub trait SmartIndex<Index> where Self: Dimension {
    fn index(&self, x: Index, y: Index) -> usize;
}

impl<T> SmartIndex<i32> for T where T: Dimension {
    fn index(&self, x: i32, y: i32) -> usize {
        let w = self.width() as i32;
        let h = self.height() as i32;
        let x = (x + w) % w;
        let y = (y + h) % h;
        self.at(x as usize, y as usize)
    }
}

impl<T> SmartIndex<usize> for T where T: Dimension {
    fn index(&self, x: usize, y: usize) -> usize {
        self.at(x, y)
    }
}