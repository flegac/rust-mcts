use itertools::Itertools;

use board::goboard::GoBoard;
use stones::grouprc::GoGroupRc;
use stones::stone::Stone;

pub struct Go {}


impl Go {
    pub fn count_stones(stone: Stone, board: &GoBoard) -> usize {
        board.groups_by_stone(stone)
            .iter()
            .map(|g| g.borrow().stones())
            .sum()
    }

    pub fn count_groups(stone: Stone, board: &GoBoard) -> usize {
        board.groups_by_stone(stone).len()
    }

    pub fn count_territory(stone: Stone, board: &GoBoard) -> usize {
        board.groups_by_stone(Stone::None)
            .iter()
            .filter(|&g| Go::get_owner(board, g.clone()) == stone)
            .map(|g| g.borrow().stones())
            .sum()
    }

    pub fn get_owner(board: &GoBoard, group: GoGroupRc) -> Stone {
        assert!(group.borrow().stone == Stone::None);

        let adjacents = group.borrow().adjacent_cells(board);
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
}

