use std::fmt::Display;
use std::iter::FromIterator;

use itertools::{Itertools, zip};

use board::group_access::GroupAccess;
use board::stones::grouprc::GoGroupRc;
use display::display::GoDisplay;
use display::goshow::GoShow;
use display::range::Range2;
use rust_tools::screen::dimension::Dimension;
use rust_tools::screen::drawer::Drawer;
use rust_tools::screen::screen::Screen;

use crate::board::go_state::GoState;
use graph_lib::topology::Topology;

pub struct BoardMap<T> {
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) map: Vec<Option<T>>,
    pub(crate) cell_size: usize,
}

impl BoardMap<GoGroupRc> {
    pub fn from_board<T>(board: &GoState, cell_size: usize) -> BoardMap<T> {
        // log::info!("BOARDMAP::FROM_BOARD");
        let width = board.gg.goban().size;
        let height = board.gg.goban().size;
        let size = width * height;
        let mut res = BoardMap {
            width,
            height,
            map: Vec::with_capacity(size),
            cell_size,
        };
        for offset in 0..size {
            res.map.push(None);
        }
        res
    }

    pub fn new(board: &GoState, cell_size: usize) -> BoardMap<GoGroupRc> {
        let mut res = BoardMap::from_board(board, cell_size);
        for i in 0..board.gg.goban().vertex_number() {
            res.map.insert(i,Some(board.gg.group_at(i).clone()));
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
            cell_size: self.cell_size,
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
    pub fn init_screen(&self, range: &Range2) -> Screen {
        let cell_size = self.cell_size;
        let w = range.x().len() * cell_size + 4;
        let h = range.y().len() + 3;
        let mut screen = Screen::new(w, h);
        let sep = vec!['-'; cell_size];
        for (x, &y) in iproduct!(range.x(), &[0, h - 2]) {
            screen.put_slice(screen.at(3 + cell_size * x, y), sep.as_slice());
        }
        for (&x, y) in iproduct!(&[2, w - 1], range.y()) {
            screen.put(screen.at(x, y + 1), '|');
        }
        for (&x, &y) in iproduct!(&[2, w - 1], &[0, h - 2]) {
            screen.put(screen.at(x, y), '+');
        }
        for y in range.y() {
            screen.put_str(screen.at(0, y + 1), &GoDisplay::line(y));
        }
        for x in range.x() {
            screen.put_str(screen.at(1 + cell_size + x * cell_size, h - 1), &GoDisplay::column(x));
        }
        screen
    }

    pub(crate) fn write_screen(&self, range: &Range2) -> Screen {
        let cell_size = self.cell_size;
        let mut screen = self.init_screen(range);
        for (x, y) in iproduct!(range.x(), range.y()) {
            let y_off = y + 1;
            let x_off = x * cell_size + 3;
            match self.get(x, y) {
                None => {
                    let delta = cell_size - 1;
                    screen.put(screen.at(x_off + delta, y_off), '.');
                }
                Some(data) => {
                    let text = data.to_string();
                    if text.len() > cell_size {
                        let delta = text.len() - cell_size;
                        screen.put_str(screen.at(x_off + delta, y_off), &text[delta..]);
                    } else {
                        let delta = cell_size - text.len();
                        screen.put_str(screen.at(x_off + delta, y_off), &text);
                    }
                }
            }
        }
        screen
    }
}
