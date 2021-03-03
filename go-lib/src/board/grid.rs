use std::hash::Hash;
use std::iter::FromIterator;

use bit_set::BitSet;

use graph_lib::topology::Topology;

pub type GoCell = usize;

#[derive(Clone)]
pub struct Grid {
    pub size: usize,
    cells: BitSet,
    links: Vec<BitSet>,
}


impl Grid {
    pub fn new(size: usize) -> Self {
        let cells = 0..(size * size);
        Grid {
            size,
            cells: BitSet::from_iter(cells.clone()),
            links: cells.clone()
                .map(|c| Grid::links(size, c))
                .collect(),
        }
    }

    pub fn cell(&self, x: usize, y: usize) -> GoCell {
        x + y * self.size
    }

    pub fn xy(&self, cell: GoCell) -> (usize, usize) {
        let x = cell as usize % self.size;
        let y = cell as usize / self.size;
        (x, y)
    }

    fn links(size: usize, cell: GoCell) -> BitSet {
        let size = size as i32;
        let limit = (size * size) as i32;
        let c = cell as i32;

        let same_line1 = |x: i32| (x % size - c % size).abs() <= 1;
        let in_board = |x: i32| x >= 0 && x < limit;

        let res = BitSet::from_iter([c - 1, c + 1, c - size, c + size].iter()
            .filter(|&x| in_board(*x))
            .filter(|&x| same_line1(*x))
            .map(|&x| x as usize));

        assert!(res.len() <= 4);
        res
    }

    fn diagonals(size: usize, cell: GoCell) -> BitSet {
        let size = size as i32;
        let limit = (size * size) as i32;
        let c = cell as i32;

        let same_line1 = |x: i32| (x % size - c % size).abs() <= 1;
        let in_board = |x: i32| x >= 0 && x < limit;

        let res = BitSet::from_iter([c - 1 - size, c + 1 - size, c - 1 + size, c + 1 + size].iter()
            .filter(|&x| in_board(*x))
            .filter(|&x| same_line1(*x))
            .map(|&x| x as usize));
        assert!(res.len() <= 4);
        res
    }
}


impl Topology for Grid {
    fn vertices(&self) -> &BitSet {
        &self.cells
    }

    fn edges(&self, v: usize) -> &BitSet {
        &self.links[v]
    }
}


#[cfg(test)]
mod tests {
    use std::ops::Deref;
    use std::sync::Arc;

    use bit_set::BitSet;

    use board::go_state::GoState;
    use board::grid::Grid;
    use graph_lib::algo::flood::Flood;
    use graph_lib::graph::GFlood;
    use graph_lib::topology::Topology;

    #[test]
    fn grid_id_conversion() {
        let goban = Grid::new(7);


        goban.apply(|c| {
            let (x, y) = goban.xy(c);
            let c2 = goban.cell(x, y);
            let (x2, y2) = goban.xy(c2);

            assert_eq!(c, c2);
            assert_eq!(x, x2);
            assert_eq!(y, y2);
        });
    }
}
