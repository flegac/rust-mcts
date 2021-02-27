pub trait Dimension {
    fn width(&self) -> usize;
    fn height(&self) -> usize;

    fn transpose(&mut self);

    fn is_mirror(&self)->bool;

    fn at(&self, x: usize, y: usize) -> usize {
        (x + y * self.width()) as usize
    }

    fn xy(&self, offset: usize) -> (usize, usize) {
        let x = offset % self.width();
        let y = offset / self.width();
        (x, y)
    }
}


pub trait ScreenIndex<Index> where Self: Dimension {
    fn index(&self, x: Index, y: Index) -> usize;
}

impl<T> ScreenIndex<i32> for T where T: Dimension {
    fn index(&self, x: i32, y: i32) -> usize {
        let w = self.width() as i32;
        let h = self.height() as i32;
        let x = (x + w) % w;
        let y = (y + h) % h;
        self.at(x as usize, y as usize)
    }
}

impl<T> ScreenIndex<usize> for T where T: Dimension {
    fn index(&self, x: usize, y: usize) -> usize {
        self.at(x, y)
    }
}

pub trait Cursor where Self: Dimension {
    fn offset(&self) -> usize;
    fn move_to(&mut self, offset: usize);

    fn cursor(&self) -> (usize, usize) {
        self.xy(self.offset())
    }
}

#[cfg(test)]
mod tests {
    use screen::dimension::{Cursor, Dimension};
    use screen::drawer::Drawer;

    use crate::screen::dimension::ScreenIndex;
    use crate::screen::screen::Screen;

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
    fn test_mirror() {
        let mut scr = Screen::new(40, 25);
        let mut x = Screen::from_string("La vie est belle !");


        scr.move_to(scr.index(7, 2));
        scr.draw(&x);

        x.transpose();
        scr.move_to(scr.index(3, 3));
        scr.draw(&x);

        x.transpose();
        scr.move_to(scr.index(9, 4));
        scr.draw(&x);

        let mut scr = scr.border();
        scr.show();
        scr.transpose();
        scr.show()

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
                scr.index(x as i32 - w as i32, y as i32 - h as i32)
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
