#[derive(Copy, Clone, Eq, PartialEq, PartialOrd)]
pub struct Dim {
    width: usize,
    height: usize,
}

impl Dim {
    pub fn new(width: usize, height: usize) -> Dim {
        Dim { width, height }
    }
}

impl Dimension for Dim {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }
}

pub trait Dimension {
    fn width(&self) -> usize;
    fn height(&self) -> usize;

    fn at(&self, x: usize, y: usize) -> usize {
        (x + y * self.width()) as usize
    }

    fn xy(&self, offset: usize) -> (usize, usize) {
        let x = offset % self.width();
        let y = offset / self.width();
        (x, y)
    }
}

#[cfg(test)]
mod tests {
    use screen::dimension::Dimension;
    use screen::drawer::Drawer;
    use screen::screen::Screen;
    use screen::smart_index::SmartIndex;

    #[test]
    fn test_dimension() {
        let mut scr = Screen::new(17, 13);

        for x in 0..scr.width() {
            for y in 0..scr.height() {
                let offset = scr.at(x, y);
                let xy = scr.xy(offset);
                let offset2 = scr.at(xy.0, xy.1);
                assert_eq!(x, xy.0);
                assert_eq!(y, xy.1);
                assert_eq!(offset, offset2);
            }
        }
    }

    #[test]
    fn test_index() {
        let w = 17;
        let h = 13;
        let scr = Screen::new(w, h);

        for expected in 0..(w * h) {
            let (x, y) = scr.xy(expected);
            let actuals = [
                scr.index(x as usize, y as usize),
                scr.index(x as i32, y as i32),
                scr.index(x as i32 - w as i32, y as i32),
                scr.index(x as i32, y as i32 - h as i32),
                scr.index(x as i32 - w as i32, y as i32 - h as i32),
            ];

            print!("{},{} :", x, y);
            for &actual in actuals.iter() {
                print!(" {}", actual);
                assert_eq!(actual, expected);
            }
            println!();
        }
    }

    #[test]
    fn test_cursor() {
        let w = 17;
        let h = 13;
        let scr = Screen::new(w, h);
    }
}
