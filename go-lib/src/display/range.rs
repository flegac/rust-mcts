#[derive(Debug)]
pub struct Range(usize, usize);

impl Range {
    fn empty() -> Range {
        Self(usize::max_value(), usize::min_value())
    }
    fn new(a: usize, b: usize) -> Range {
        Self::empty().merge(a).merge(b)
    }

    fn merge(&self, value: usize) -> Self {
        Range(self.0.min(value),
              self.1.max(value))
    }

    pub fn size(&self) -> usize {
        self.1 - self.0
    }

    pub fn iter(&self) -> std::ops::Range<usize> {
        self.0..(self.1 + 1)
    }
}

#[derive(Debug)]
pub struct Range2 {
    pub x: Range,
    pub y: Range,
}

impl Range2 {
    pub fn board(size: usize) -> Range2 {
        Range2 {
            x: Range(0, size - 1),
            y: Range(0, size - 1),
        }
    }

    pub fn empty() -> Range2 {
        Range2 {
            x: Range::empty(),
            y: Range::empty(),
        }
    }

    pub fn size(&self) -> usize {
        self.x.size() * self.y.size()
    }

    pub fn merge(&self, xy: (usize, usize)) -> Self {
        Range2 {
            x: self.x.merge(xy.0),
            y: self.y.merge(xy.1),
        }
    }
}