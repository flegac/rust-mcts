use std::io::Cursor;

use bit_set::BitSet;
use image::{ImageBuffer, ImageFormat, Rgb};
use image::RgbImage;

use board::go_state::GoState;
use board::grid::{GoCell, Grid};
use board::stones::stone::Stone;

#[derive(Debug)]
pub struct BoardCollection {
    data: RgbImage,
    width: usize,
    height: usize,
}

impl BoardCollection {
    const BLACK: Rgb<u8> = Rgb([255, 0, 0]);
    const WHITE: Rgb<u8> = Rgb([0, 255, 0]);
    const NONE: Rgb<u8> = Rgb([0, 0, 255]);

    pub fn new(width: usize, height: usize) -> Self {
        BoardCollection {
            data: RgbImage::new(width as u32, height as u32),
            width: width,
            height: height,
        }
    }

    pub fn insert(&mut self, x: usize, y: usize, stone: Stone) {
        self.data.put_pixel(x as u32, y as u32, self.stone_color(stone));
    }

    fn stone_color(&self, stone: Stone) -> Rgb<u8> {
        match stone {
            Stone::Black => BoardCollection::BLACK,
            Stone::White => BoardCollection::WHITE,
            Stone::None => BoardCollection::NONE,
        }
    }

    #[inline]
    fn size(&self) -> usize {
        self.width * self.height
    }

    pub fn write_image(&self, path: &str) {
        self.data.save(path).unwrap();
    }
}


#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use board::grid::Grid;
    use board::raw_board::BoardCollection;
    use board::stones::stone::Stone;

    #[test]
    fn test_raw_board() {
        let mut board = BoardCollection::new(&Grid::new(4));
        board.insert(0, 1, Stone::Black);
        board.insert(2, 2, Stone::White);
        board.insert(3, 1, Stone::Black);
        board.write_image("output.png");

        println!("{:?}", board);
    }
}