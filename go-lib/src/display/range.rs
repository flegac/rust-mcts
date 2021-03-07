use std::ops::{Range, RangeBounds};

#[derive(Debug)]
pub struct Range2 {
    pub _x: Range<usize>,
    pub _y: Range<usize>,
}

impl Range2 {
    pub fn board(size: usize) -> Range2 {
        Range2 {
            _x: 0..size,
            _y: 0..size,
        }
    }

    pub fn empty() -> Range2 {
        Range2 {
            _x: 0..0,
            _y: 0..0,
        }
    }
    pub fn x(&self) -> Range<usize> {
        self._x.clone()
    }
    pub fn y(&self) -> Range<usize> {
        self._y.clone()
    }
    pub fn size(&self) -> usize {
        self._x.len() * self._y.len()
    }

    pub fn merge(&self, xy: (usize, usize)) -> Self {
        Range2 {
            _x: Self::merge_range(&self.x(), xy.0),
            _y: Self::merge_range(&self.y(), xy.1),
        }
    }

    fn merge_range(rr: &Range<usize>, value: usize) -> Range<usize> {
        let a: usize = match rr.clone().min() {
            None => value,
            Some(x) => x.min(value)
        };
        let b: usize = match rr.clone().max() {
            None => value,
            Some(x) => x.max(value)
        };
        a..b
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use display::range::Range2;

    #[test]
    fn test() {
        let r = Range2::empty();

        assert_eq!(r.size(), 0);
        assert_eq!(r.x().collect_vec(), vec![]);
        assert_eq!(r.y().collect_vec(), vec![]);
    }
}