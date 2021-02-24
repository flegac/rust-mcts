use core::fmt;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

use bit_set::BitSet;
use fixed_typed_arena::Arena;
use itertools::Itertools;

use board::grid::{GoCell, Grid};
use board::stats_board::BoardStats;
use graph_lib::graph::{Graph, Vert};
use stones::group::GoGroup;
use stones::grouprc::GoGroupRc;
use stones::stone::Stone;

pub struct GoBoard {
    arena: Arena<GoGroup>,
    pub(crate) goban: Grid,
    groups: HashMap<GoCell, GoGroupRc>,
    pub(crate) stats: BoardStats,
    pub stone: Stone,
    pub(crate) empty_cells: GoGroup,
}

impl GoBoard {
    pub fn new(goban: Grid) -> Self {
        let empty_cells = GoGroup {
            stone: Stone::None,
            cells: goban.vertices().clone(),
            liberties: 0,
        };
        let mut board = GoBoard {
            arena: Arena::new(),
            goban,
            groups: HashMap::new(),
            stats: BoardStats::new(),
            stone: Stone::Black,
            empty_cells,
        };


        board.update_group(board.new_group(GoGroup {
            stone: Stone::None,
            cells: board.goban.vertices().clone(),
            liberties: 0,
        }));
        board
    }

    #[inline(always)]
    pub fn group_at(&self, cell: &GoCell) -> GoGroupRc {
        self.groups.get(&cell).unwrap().clone()
    }

    #[inline(always)]
    pub fn stone_at(&self, cell: &GoCell) -> Stone {
        self.group_at(cell).borrow().stone
    }
    #[inline(always)]
    pub fn groups_by_stone(&self, stone: Stone) -> Vec<GoGroupRc> {
        self.groups.values()
            .filter(|&g| g.borrow().stone == stone)
            .unique()
            .map(|g| g.clone())
            .collect_vec()
    }

    pub fn end_game(&self) -> bool {
        let limit = self.vertices().len();
        self.stats.round > limit || self.stats.none.groups == 0
    }

    pub fn reset(&mut self) {
        self.groups.clear();
        self.stats = BoardStats::new();
        self.stone = Stone::Black;
        self.empty_cells = GoGroup {
            stone: Stone::None,
            cells: self.goban.vertices().clone(),
            liberties: 0,
        };
        self.update_group(self.new_group(GoGroup {
            stone: Stone::None,
            cells: self.goban.vertices().clone(),
            liberties: 0,
        }));
    }

    pub fn place_stone(&mut self, cell: GoCell, stone: Stone) {
        assert!(self.stone_at(&cell) == Stone::None);

        log::trace!("board:\n{}", self);
        log::trace!("PLACE STONE: {} @ {:?}", stone, self.goban.xy(cell));

        let new_group = self.new_group(GoGroup::from_cell(stone, cell));
        let old = self.group_at(&cell).clone();
        old.borrow_mut().remove_group(&new_group.borrow());
        self.stats.rem_group(old.borrow().deref());

        for part in old.borrow_mut().split(&self.goban) {
            self.update_group(self.new_group(part));
        }


        self.empty_cells.remove_group(new_group.borrow().deref());


        // update board with new group
        self.goban.edges(cell)
            .iter()
            .filter(|c| self.stone_at(c) == stone)
            .map(|c| self.group_at(&c))
            .sorted()
            .dedup()
            .for_each(|g: GoGroupRc| {
                new_group.borrow_mut().add_group(g.borrow().deref());
                self.stats.rem_group(&g.borrow());
            });
        self.update_group(new_group.clone());

        // kill groups
        let deads: Vec<GoGroupRc> = self.goban.edges(cell)
            .iter()
            .filter(|c| self.stone_at(c) == stone.switch())
            .map(|c| self.group_at(&c))
            .sorted()
            .dedup()
            .collect_vec();

        for g in deads {
            g.borrow_mut().update_liberties(self);
            if g.borrow().is_dead() {
                self.stats.capture_group(g.borrow_mut().deref_mut());
                self.empty_cells.add_group(g.borrow().deref());
            }
        }

        //FIXME: do not allow this case to happen !
        new_group.borrow_mut().update_liberties(self);
        if new_group.borrow().is_dead() {
            log::trace!("AUTOKILL MOVE! {}", new_group);
            self.stats.capture_group(new_group.borrow_mut().deref_mut());
            self.empty_cells.add_group(new_group.borrow().deref());
        }

        self.stats.round += 1;
        assert_eq!(self.stats.round, self.stats.compute_round());

        //TODO: remove this when all is ok !
        // self.stats.assert_eq(&BoardStats::new(self));
        assert_eq!(self.empty_cells.stones(), self.stats.none.stones);
    }

    pub fn update_score<F>(&mut self, scorer: F)
        where F: Fn(Stone, &GoBoard) -> usize
    {
        self.stats.black.territory = scorer(Stone::Black, self);
        self.stats.white.territory = scorer(Stone::White, self);
    }

    pub fn score(&self, stone: Stone) -> usize {
        let stats = self.stats.for_stone(stone);
        let territory = stats.territory;
        let captures = stats.captured;
        territory + captures
    }

    fn update_group(&mut self, group: GoGroupRc) {
        for c in group.borrow().cells.iter() {
            self.groups.insert(c, group.clone());
        }
        self.stats.add_group(group.borrow().deref());
    }

    fn new_group(&self, group: GoGroup) -> GoGroupRc {
        // self.arena.alloc(group)
        GoGroupRc::from(group)
    }
}


impl Graph for GoBoard {
    #[inline]
    fn vertices(&self) -> &BitSet<u32> {
        self.goban.vertices()
    }
    #[inline]
    fn edges(&self, v: usize) -> &BitSet<u32> {
        self.goban.edges(v)
    }
    #[inline]
    fn flood<F>(&self, cell: usize, test: &F) -> BitSet<u32>
        where F: Fn(Vert) -> bool {
        self.goban.flood(cell, &test)
    }
}

impl fmt::Display for GoBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}\n{}",
               self.draw_board(),
               self.stats.score_string(),
               self.stats
        )
    }
}


const BIG_A: usize = 'A' as usize;

impl GoBoard {
    fn draw_board(&self) -> String {
        let size = self.goban.size;
        let mut res = String::new();
        self.draw_line(&mut res, true);
        self.draw_line_separator(&mut res);
        let a = 'a' as usize;
        for y in 0..size {
            res.push_str(format!("{} | ", char::from((y + a) as u8)).as_str());
            for x in 0..size {
                let g = self.stone_at(&self.goban.cell(x, y));
                res.push_str(format!("{} ", g).as_str());
            }
            res.push_str(format!("| {}", char::from((y + a) as u8)).as_str());

            res.push_str("\n");
        }
        self.draw_line_separator(&mut res);
        self.draw_line(&mut res, true);
        res
    }
    fn draw_line_separator(&self, res: &mut String) {
        let size = self.goban.size;

        res.push_str("  + ");
        for _x in 0..size {
            res.push_str("--");
        }
        res.push_str("+  \n");
    }

    fn draw_line(&self, res: &mut String, with_side: bool) {
        let size = self.goban.size;
        match with_side {
            true => res.push_str(format!("[{}] ", self.stone).as_str()),
            false => res.push_str("    ")
        }
        for x in 0..size {
            res.push_str(format!("{} ", char::from((x + BIG_A) as u8)).as_str());
        }
        match with_side {
            true => res.push_str(format!("[{}]", self.stone).as_str()),
            false => res.push_str("   ")
        }
        res.push_str("\n");
    }
}


#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use bit_set::BitSet;
    use rpool::{Pool, Poolable, PoolScaleMode};

    use board::goboard::GoBoard;
    use board::grid::Grid;
    use graph_lib::graph::Graph;
    use stones::group::GoGroup;
    use stones::grouprc::GoGroupRc;
    use stones::stone::Stone;

    #[test]
    fn stone_groups() {
        let goban = Grid::new(7);
        let board = GoBoard::new(goban);

        let mut cells = BitSet::new();
        for cell in &[
            board.goban.cell(0, 0),
            board.goban.cell(0, 3),
            board.goban.cell(3, 0)
        ] {
            cells.insert(*cell);
        }

        let group = board.new_group(GoGroup {
            stone: Stone::None,
            cells,
            liberties: 0,
        });

        assert_eq!(group.borrow().stones(), 3);
    }

    #[test]
    fn board_cell_id() {
        let goban = Grid::new(7);

        for c in goban.vertices().iter() {
            let (x, y) = goban.xy(c);
            let c2 = goban.cell(x, y);
            let (x2, y2) = goban.xy(c2);

            assert_eq!(c, c2);
            assert_eq!(x, x2);
            assert_eq!(y, y2);
        }
    }


    #[test]
    fn test_group_splitting() {
        let board = GoBoard::new(Grid::new(7));
        let test1 = |c| {
            let (x, y) = board.goban.xy(c);
            x == 0
        };
        let test2 = |c| {
            let (x, y) = board.goban.xy(c);
            x == 2
        };
        let mut cells1 = board.flood(board.goban.cell(0, 0), &test1);
        cells1.union_with(&board.flood(board.goban.cell(2, 0), &test2));
        let g = board.new_group(GoGroup {
            stone: Stone::White,
            cells: cells1,
            liberties: 0,
        });
        println!("big group: {}", g);


        let gg = g.borrow_mut().split(&board);

        for ga in gg {
            println!("- {}", ga)
        }
    }
}
