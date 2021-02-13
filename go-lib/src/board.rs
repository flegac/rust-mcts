use core::fmt;
use std::cell::{Ref, RefCell, RefMut};
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::iter::Map;
use std::ops::Range;
use std::rc::Rc;

use itertools::{iproduct, Itertools, Product};

use mcts_lib::mymcts::MyMcts;
use stones::group::{GoGroup, GoGroupRc};
use stones::stone::Stone;

use crate::action::GoAction;
use crate::constants::GOBAN_SIZE;

pub type GoCell = usize;


pub(crate) struct GoBoard<> {
    board: HashMap<GoCell, GoGroupRc>,
}


impl GoBoard {
    pub(crate) fn new() -> Self {
        let mut board = GoBoard {
            board: HashMap::new(),
        };
        board.reset();
        board
    }

    pub(crate) fn reset(&mut self) {
        let group = GoGroupRc::new(Stone::None)
            .with_cells(self.cells().as_slice());
        self.update_group(
            &group);
    }

    pub(crate) fn update_group(&mut self, stones: &GoGroupRc) {
        for c in stones.borrow().cells.iter() {
            self.board.insert(c, stones.clone());
        }
    }

    pub(crate) fn lines(&self) -> Vec<Vec<usize>> {
        (0..GOBAN_SIZE)
            .map(|y| (0..GOBAN_SIZE)
                .map(|x| self.cell(x, y))
                .collect_vec())
            .collect_vec()
    }

    pub(crate) fn cells(&self) -> Vec<usize> {
        let data = (iproduct![0..GOBAN_SIZE, 0..GOBAN_SIZE])
            .into_iter()
            .map(|(x, y)| self.cell(x, y)).collect_vec();
        data
    }

    pub(crate) fn cell(&self, x: usize, y: usize) -> GoCell {
        x * GOBAN_SIZE + y
    }

    pub(crate) fn xy(&self, cell: GoCell) -> (usize, usize) {
        let x = cell as usize / GOBAN_SIZE;
        let y = cell as usize % GOBAN_SIZE;
        (x, y)
    }

    pub(crate) fn adjacents(&self, cell: GoCell) -> Vec<GoCell> {
        let (x0, y0) = self.xy(cell);
        (iproduct![0..3,0..3])
            .into_iter()
            .filter(|(dx, dy)| *dx == 1 || *dy == 1)
            .filter(|(dx, dy)| *dx != 1 || *dy != 1)
            .map(|(dx, dy)| (x0 + dx, y0 + dy))
            .filter(|(x, y)| *x > 0 && *x <= GOBAN_SIZE)
            .filter(|(x, y)| *y > 0 && *y <= GOBAN_SIZE)
            .map(|(x, y)| (x - 1, y - 1))
            .map(|(x, y)| self.cell(x, y))
            .collect_vec()
    }

    pub(crate) fn update(&mut self, cell: GoCell, value: Stone) {
        // creating new group
        let cells = vec![cell];
        // TODO: find adjacent groups (on adjacent cells) & fusion them together if appropriate
        let gg = GoGroupRc::new(value).with_cells(cells.as_slice());


        // removing cells from old group
        let mut old = self.board.get(&cell).unwrap();
        old.borrow_mut().remove_group(&gg.borrow());

        println!("new: {} adjacent:{:?}", gg, self.adjacents(cell));
        println!("old: {}", old);


        //TODO: check if old group connectivity & split it if needed

        //updating board with new group
        self.update_group(&gg);
    }


    pub(crate) fn count_stones(&self, stone: Stone) -> usize {
        // self.groups.into_iter()
        //     .filter(|g| g.stone == stone)
        //     .map(|g| g.cells.len())
        //     .sum()
        self.board.iter()
            .filter(|(c, g)| g.borrow().stone == stone)
            .count()
    }

    pub(crate) fn actions(&self) -> Vec<GoAction> {
        // self.groups.into_iter()
        //     .filter(|g| g.stone == None)
        //     .flat_map(|g| g.cells.iter())
        //     .map(|c| GoAction::play_at(c))
        //     .collect_vec()
        self.cells()
            .iter()
            .filter(|&c| self.board.get(c).unwrap().borrow().stone == Stone::None)
            .map(|&c| GoAction::play_at(c))
            .collect_vec()
    }
}


impl fmt::Display for GoBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = String::new();

        for x in 0..GOBAN_SIZE {
            for y in 0..GOBAN_SIZE {
                match self.board.get(&self.cell(x, y)) {
                    None => {
                        res.push_str(".");
                    }
                    Some(g) => {
                        res.push_str(&g.borrow().stone.to_string());
                    }
                }
                res.push_str(" ");
            }
            res.push_str("\n");
        }
        write!(f, "{}", res)
    }
}
