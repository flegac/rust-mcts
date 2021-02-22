use core::fmt;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::ops::Deref;

use bit_set::BitSet;
use fixed_typed_arena::Arena;
use itertools::{Itertools, sorted};

use board::grid::{GoCell, Grid};
use board::stats_board::BoardStats;
use stones::group::GoGroup;
use stones::grouprc::GoGroupRc;
use stones::stone::Stone;

pub(crate) struct GoBoard {
    arena: Arena<GoGroup>,
    pub(crate) goban: Grid,
    // groups: Vec<GoGroupRc>,
    pub(crate) groups: HashMap<GoCell, GoGroupRc>,
    pub(crate) stats: BoardStats,
}

impl GoBoard {
    pub fn new(goban: Grid) -> Self {
        // let cell_number = goban.size * goban.size;
        let mut board = GoBoard {
            arena: Arena::new(),
            goban,
            // groups: Vec::with_capacity(cell_number),
            groups: HashMap::new(),
            stats: BoardStats::init(),
        };

        let mut new_group = board.board_group();
        board.update_board_with_group(&mut new_group);
        board
    }


    pub fn cell_group(&self, stone: Stone, cell: GoCell) -> GoGroupRc {
        let mut cells = BitSet::new();
        cells.insert(cell);
        let g = self.new_group(stone, cells, 4);
        g
    }

    pub fn board_group(&self) -> GoGroupRc {
        self.new_group(Stone::None, self.goban.cells.clone(), 0)
    }

    pub fn new_group(&self, stone: Stone, cells: BitSet, liberties: usize) -> GoGroupRc {
        // self.arena.alloc(GoGroup {stone, cells, liberties})
        GoGroupRc::new(stone, cells, liberties)
    }


    pub fn place_stone(&mut self, cell: GoCell, stone: Stone) {
        log::trace!("board:\n{}", self);
        log::debug!("PLACE STONE: {} @ {:?}", stone, self.goban.xy(cell));

        let new_group = self.cell_group(stone, cell);
        let old = self.group_at(&cell).clone();
        old.borrow_mut().remove_group(&new_group.borrow());
        self.stats.rem_group(&old);
        for part in self.split(old) {
            self.update_board_with_group(&part);
        }

        // update board with new group
        self.goban.edges[cell]
            .iter()
            .filter(|c| self.stone_at(c) == stone)
            .map(|c| self.group_at(&c))
            .sorted()
            .dedup()
            .for_each(|g| {
                new_group.borrow_mut().add_group(g.borrow().deref());
                self.stats.rem_group(&g);
            });
        self.update_board_with_group(&new_group);

        // kill groups
        let deads = self.goban.edges[cell]
            .iter()
            .filter(|c| self.stone_at(c) == stone.switch())
            .map(|c| self.group_at(&c))
            .sorted()
            .dedup()
            .collect_vec();

        for g in deads {
            self.update_group_liberties(&g);
            if self.is_dead(&g) {
                self.capture_group(&g);
            }
        }

        //FIXME: do not allow this case to happen !
        self.update_group_liberties(&new_group);
        if self.is_dead(&new_group) {
            log::debug!("AUTOKILL MOVE! {}", new_group);
            self.capture_group(&new_group);
        }

        //TODO: remove this when all is ok !
        // self.stats.assert_eq(&BoardStats::new(self));
    }


    pub fn group_at(&self, cell: &GoCell) -> GoGroupRc {
        self.groups.get(&cell).unwrap().clone()
    }


    pub fn stone_at(&self, cell: &GoCell) -> Stone {
        self.group_at(cell).borrow().stone
    }

    fn split(&self, g: GoGroupRc) -> Vec<GoGroupRc> {
        let mut res = vec![];

        while !g.borrow().is_empty() {
            let g1 = self.next_split(&g);
            g.borrow_mut().remove_group(&g1.borrow());
            res.push(g1);
        }
        res
    }


    fn capture_group(&mut self, group: &GoGroupRc) {
        self.stats.rem_group(&group);
        match group.borrow().stone {
            Stone::None => {}
            Stone::Black => self.stats.black.captured += group.borrow().size(),
            Stone::White => self.stats.white.captured += group.borrow().size(),
        }
        group.borrow_mut().set_stone(Stone::None);
        self.stats.add_group(&group);

        // the stones has been counted twice for None group
        self.stats.none.stones -= group.borrow().cells.len();
    }

    fn is_dead(&self, group: &GoGroupRc) -> bool {
        group.borrow().liberties == 0
    }


    pub fn adjacent_cells(&self, group: GoGroupRc) -> BitSet {
        let mut adjacents = BitSet::new();
        for c in group.borrow().cells.iter() {
            adjacents.union_with(&self.goban.edges[c]);
        }
        adjacents.difference_with(&group.borrow().cells);
        adjacents
    }


    fn count_liberties(&self, group: &GoGroupRc) -> usize {
        let mut adjacents = self.adjacent_cells(group.clone());

        let mut liberties = BitSet::new();
        for x in adjacents.iter()
            .filter(|c| self.group_at(c).borrow().stone == Stone::None) {
            liberties.insert(x);
        }
        liberties.len()
    }

    fn update_group_liberties(&self, group: &GoGroupRc) {
        group.borrow_mut().liberties = self.count_liberties(group);
    }


    fn update_board_with_group(&mut self, group: &GoGroupRc) {
        for c in group.borrow().cells.iter() {
            self.groups.insert(c, group.clone());
        }
        self.stats.add_group(&group);
    }

    fn next_split(&self, group: &GoGroupRc) -> GoGroupRc {
        let to_visit = &group.borrow().cells;
        let test = |c: GoCell| to_visit.contains(c);

        let cell = to_visit.iter().next().unwrap();
        let cells = self.goban.flood(cell, &test);
        let liberties = 0;
        self.new_group(group.borrow().stone, cells, liberties)
    }
}


impl fmt::Display for GoBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let size = self.goban.size;

        let mut res = String::new();
        for y in 0..size {
            for x in 0..size {
                let g = self.stone_at(&self.goban.cell(x, y));
                res.push_str(format!("{} ", g).as_str());
            }
            res.push_str("\n");
        }
        write!(f, "{}", format!("{}{}\n{}",
                                res,
                                self.stats.score_string(),
                                self.stats
        ))
    }
}
