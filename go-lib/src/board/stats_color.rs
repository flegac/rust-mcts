use std::fmt;

use itertools::Itertools;

use board::goboard::GoBoard;
use stones::grouprc::GoGroupRc;
use stones::stone::Stone;

#[derive(Clone, Copy, Eq, PartialEq)]
pub(crate) struct ColorStats {
    pub(crate) stone: Stone,
    pub(crate) groups: usize,
    pub(crate) stones: usize,
    pub(crate) captured: usize,
    pub(crate) territory: usize,
}

impl ColorStats {
    pub(crate) fn init(stone: Stone) -> ColorStats {
        ColorStats {
            stone,
            stones: 0,
            groups: 0,
            captured: 0,
            territory: 0,
        }
    }
    pub(crate) fn new(stone: Stone, board: &GoBoard) -> ColorStats {
        ColorStats {
            stone,
            stones: ColorStats::count_stones(stone, board),
            groups: ColorStats::count_groups(stone, board),
            captured: 0,
            territory: 0,
        }
    }
    pub(crate) fn assert_eq(&self, other: &ColorStats) {
        assert_eq!(self.stones, other.stones, "[{}] stones", self.stone);
        assert_eq!(self.groups, other.groups, "[{}] groups", self.stone);
    }

    pub fn count_stones(stone: Stone, board: &GoBoard) -> usize {
        board.groups.values()
            .filter(|&g| g.borrow().stone == stone)
            .unique()
            .map(|g| g.borrow().size())
            .sum()
    }

    pub fn count_groups(stone: Stone, board: &GoBoard) -> usize {
        board.groups.values()
            .filter(|&g| g.borrow().stone == stone)
            .unique()
            .count()
    }


    fn get_owner(board: &GoBoard, group: GoGroupRc) -> Stone {
        assert!(group.borrow().stone == Stone::None);

        let adjacents = board.adjacent_cells(group.clone());
        let border = adjacents.iter()
            .map(|c| board.stone_at(&c))
            .unique()
            .collect_vec();

        let white = border.contains(&Stone::White);
        let black = border.contains(&Stone::Black);

        let owner = if white && black {
            Stone::None
        } else if white {
            Stone::White
        } else {
            Stone::Black
        };
        owner
    }

    pub fn count_territory(stone: Stone, board: &GoBoard) -> usize {
        board.groups.values()
            .filter(|&g| g.borrow().stone == Stone::None)
            .unique()
            .filter(|&g| ColorStats::get_owner(board, g.clone()) == stone)
            .map(|g| g.borrow().size())
            .sum()
    }
}


impl fmt::Display for ColorStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("{}: {} stones, {} groups",
                                &self.stone,
                                &self.stones,
                                &self.groups
        ).as_str())
    }
}
