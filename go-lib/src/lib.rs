extern crate bit_set;
extern crate core;
extern crate graph_lib;
extern crate indexmap;
extern crate itertools;
extern crate log;
extern crate mcts_lib;
extern crate proc_macro;
extern crate rust_tools;

pub mod board;
pub mod sgf;
pub mod display;
pub mod mcts;
pub mod go_rules;

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use bit_set::BitSet;

    use crate::board::go_state::GoState;
    use board::grid::Grid;
    use graph_lib::topology::Topology;

    #[test]
    fn board_cell_id() {
        let goban = Grid::new(7);

        goban.apply(|c| {
            let (x, y) = goban.xy(c);
            let c2 = goban.cell(x, y);
            let (x2, y2) = goban.xy(c2);

            assert_eq!(c, c2);
            assert_eq!(x, x2);
            assert_eq!(y, y2);
        }
        );
    }
}
