use bit_set::BitSet;
use itertools::Itertools;

use board::group_access::GroupAccess;
use board::stones::grouprc::GoGroupRc;
use board::stones::stone::Stone;
use graph_lib::topology::Topology;

pub struct Go<'a, T: GroupAccess> {
    state: &'a T
}

impl<'a, T: GroupAccess> Go<'a, T> {
    pub fn new(state: &'a T) -> Go<'a, T> {
        Go { state }
    }
}


impl<'a, T: GroupAccess> Go<'a, T> {
    pub fn adjacent_cells(&self, cells: &BitSet) -> BitSet {
        let mut adjacents = BitSet::new();
        for c in cells.iter() {
            adjacents.union_with(&self.state.goban().edges(c));
        }
        adjacents.difference_with(cells);
        adjacents
    }


    pub fn count_stones(&self, stone: Stone) -> usize {
        self.state.groups_by_stone(stone)
            .iter()
            .map(|g| g.borrow().stones())
            .sum()
    }

    fn count_stones2(&self, stone: Stone) -> usize {
        self.state.goban().vertices().iter()
            .map(|c| self.state.stone_at(c))
            .filter(|&s| s == stone)
            .count()
    }


    pub fn count_groups(&self, stone: Stone) -> usize {
        self.state.groups_by_stone(stone).len()
    }

    pub fn count_territory(&self, stone: Stone) -> usize {
        match stone {
            Stone::None => 0,
            _ => self.state.groups_by_stone(Stone::None)
                .iter()
                .filter(|&g| self.get_owner(g.clone()) == stone)
                .map(|g| g.borrow().stones())
                .sum()
        }
    }

    pub fn get_owner(&self, group: GoGroupRc) -> Stone {
        assert!(group.borrow().stone == Stone::None);

        let adjacents = self.adjacent_cells(&group.borrow().cells);
        let border = adjacents.iter()
            .map(|c| self.state.stone_at(c))
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

