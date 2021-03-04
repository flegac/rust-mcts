use board::go_state::GoState;
use board::group_access::GroupAccess;
use board::stones::grouprc::GoGroupRc;
use std::fmt::Display;
use display::range::Range2;
use std::iter::FromIterator;
use display::display::GoDisplay;
use display::goshow::GoShow;

pub struct BoardMap<T> {
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) map: Vec<Option<T>>,
}

impl BoardMap<GoGroupRc> {
    pub fn new(board: &GoState) -> BoardMap<GoGroupRc> {
        let width = board.goban().size;
        let height = board.goban().size;
        let size = width * height;
        let mut res = BoardMap {
            width,
            height,
            map: Vec::with_capacity(size),
        };
        for i in 0..size {
            res.map.push(Some(board.group_at(i).clone()));
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
    pub(crate) fn map_str(&self, range: Range2, cell_size: usize) -> String {
        //TODO: clean up this trash !
        let columns = String::from_iter(
            range.x.iter()
                .map(GoDisplay::column)
                .map(|x| format!(" {} ", x))
        );
        let mut empty = vec![' '; cell_size - 1];
        empty.push('.');
        let empty_cell = String::from_iter(empty);
        let separator = String::from_iter(
            vec!['-'; (range.x.size() + 1) * cell_size]);

        let mut res = String::new();
        res.push_str(&format!("  +{}+\n", separator));
        for y in range.y.iter().rev() {
            res.push_str(&format!("{} |", GoDisplay::line(y)));
            for x in range.x.iter() {
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
            res.push_str(&format!("|\n"));
        }
        res.push_str(&format!("  +{}+\n", separator));
        res.push_str(&format!("   {}\n", columns));
        res
    }
}
