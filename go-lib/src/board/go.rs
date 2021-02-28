use bit_set::BitSet;
use graph_lib::topology::Topology;
use itertools::Itertools;

use board::go_state::GoState;
use stones::grouprc::GoGroupRc;
use stones::stone::Stone;
use groups::group_access::GroupAccess;

pub struct Go {}


impl Go {
    pub fn adjacent_cells<G: Topology>(board: &G, cells: &BitSet) -> BitSet {
        let mut adjacents = BitSet::new();
        for c in cells.iter() {
            adjacents.union_with(&board.edges(c));
        }
        adjacents.difference_with(cells);
        adjacents
    }


    pub fn count_stones(stone: Stone, board: &GoState) -> usize {
        board.groups_by_stone(stone)
            .iter()
            .map(|g| g.borrow().stones())
            .sum()
    }

    pub fn count_groups(stone: Stone, board: &GoState) -> usize {
        board.groups_by_stone(stone).len()
    }

    pub fn count_territory(stone: Stone, board: &GoState) -> usize {
        match stone {
            Stone::None => 0,
            _ => board.groups_by_stone(Stone::None)
                .iter()
                .filter(|&g| Go::get_owner(board, g.clone()) == stone)
                .map(|g| g.borrow().stones())
                .sum()
        }
    }

    pub fn get_owner(board: &GoState, group: GoGroupRc) -> Stone {
        assert!(group.borrow().stone == Stone::None);

        let adjacents = Go::adjacent_cells(board, &group.borrow().cells);
        let border = adjacents.iter()
            .map(|c| board.stone_at(c))
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

