use std::fmt::Display;
use std::iter::FromIterator;

use board::group_access::GroupAccess;
use board::stones::grouprc::GoGroupRc;
use display::display::GoDisplay;
use display::goshow::GoShow;
use display::range::Range2;
use rust_tools::screen::dimension::Dimension;
use rust_tools::screen::drawer::Drawer;
use rust_tools::screen::screen::Screen;

use crate::board::go_state::GoState;

pub struct BoardMap<T> {
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) map: Vec<Option<T>>,
}

impl BoardMap<GoGroupRc> {
    pub fn new(board: &GoState) -> BoardMap<GoGroupRc> {
        let width = board.gg.goban().size;
        let height = board.gg.goban().size;
        let size = width * height;
        let mut res = BoardMap {
            width,
            height,
            map: Vec::with_capacity(size),
        };
        for i in 0..size {
            res.map.push(Some(board.gg.group_at(i).clone()));
        }
        res
    }
}

impl<T> BoardMap<T> {
    pub fn xy(&self, cell: usize) -> (usize, usize) {
        let x = cell as usize % self.width;
        let y = cell as usize / self.width;
        (x, y)
    }
    pub fn get(&self, x: usize, y: usize) -> &Option<T> {
        &self.map[x + y * self.width]
    }
    pub fn map<U, F: Fn(&T) -> Option<U>>(self, func: F) -> BoardMap<U> {
        let size = self.width * self.height;
        let mut res = BoardMap {
            width: self.width,
            height: self.height,
            map: Vec::with_capacity(size),
        };
        for i in 0..size {
            res.map.push(match &self.map[i] {
                None => None,
                Some(data) => func(data),
            });
        }
        res
    }
}

impl<T: Display> BoardMap<T> {
    pub(crate) fn map_str(&self, range: &Range2, cell_size: usize) -> String {
        //TODO: clean up this trash !

        let mut empty = vec![' '; cell_size - 1];
        let spacer = String::from_iter(&empty);
        empty.push('.');
        let empty_cell = String::from_iter(empty);
        let columns = String::from_iter(
            range.x()
                .map(GoDisplay::column)
                .map(|x| format!("{}{}", spacer, x))
        );
        let separator = String::from_iter(
            vec!['-'; range.x().len() * cell_size]);

        let mut res = String::new();
        res.push_str(&format!("  +{}-+\n", separator));
        for y in range.y().rev() {
            res.push_str(&format!("{} |", GoDisplay::line(y)));
            for x in range.x() {
                match self.get(x, y) {
                    None => {
                        res.push_str(&empty_cell);
                    }
                    Some(data) => {
                        let text = data.to_string();
                        if text.len() > cell_size {
                            let delta = text.len() - cell_size;
                            for i in 0..delta {
                                res.push(' ');
                            }
                            res.push_str(&text[delta..]);
                        } else {
                            let delta = cell_size - text.len();
                            for i in 0..delta {
                                res.push(' ');
                            }
                            res.push_str(&text);
                        }
                    }
                }
            }
            res.push_str(&format!(" |\n"));
        }
        res.push_str(&format!("  +{}-+\n", separator));
        res.push_str(&format!("   {}\n", columns));
        res
    }

    pub(crate) fn map_screen(&self, range: &Range2, cell_size: usize) -> Screen {
        log::trace!("range: {:?}", range);
        log::trace!("range_x: {} range_y: {}", range.x().len(), range.y().len());

        if range.x().len() <=2 || range.y().len() <= 2 {
            return Screen::new(1,1);
        }

        let width = range.x().len() * cell_size + 4;
        let height = range.y().len() + 3;
        let mut screen = Screen::new(width, height);

        let mut empty = vec![' '; cell_size - 1];
        let spacer = String::from_iter(&empty);
        let columns = String::from_iter(
            range.x()
                .map(GoDisplay::column)
                .map(|x| format!("{}{}", spacer, x))
        );
        let separator = String::from_iter(
            vec!['-'; range.x().len() * cell_size]);


        //borders
        screen.put_str(screen.at(2, 0), &format!("+{}+", separator));
        screen.put_str(screen.at(2, height - 2), &format!("+{}+", separator));
        screen.put_str(screen.at(2, height - 1), &format!("{}", columns));
        for y in range.y() {
            screen.put_str(screen.at(0, y + 1), &format!("{} |", GoDisplay::line(y)));
            screen.put_str(screen.at(width - 1, y + 1), &format!("|"));
        }


        for y in range.y() {
            let y_off = y + 1;
            for x in range.x() {
                let x_off = x * cell_size + 3;
                // log::trace!("xy: {},{}", x,y);
                match self.get(x, y) {
                    None => {
                        let delta = cell_size - 1;
                        screen.put(screen.at(x_off + delta, y_off), '.');
                    }
                    Some(data) => {
                        let text = data.to_string();
                        // println!("text: [{}] at xy: {},{}", text, x, y);
                        if text.len() > cell_size {
                            let delta = text.len() - cell_size ;
                            screen.put_str(screen.at(x_off + delta, y_off), &text[delta..]);
                        } else {
                            let delta = cell_size - text.len() ;
                            screen.put_str(screen.at(x_off + delta, y_off), &text);
                        }
                    }
                }
            }
        }



        screen
    }
}
