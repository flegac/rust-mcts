extern crate bit_set;
extern crate core;
extern crate graph_lib;
extern crate itertools;
extern crate log;
extern crate mcts_lib;
extern crate proc_macro;
extern crate rust_tools;
extern crate indexmap;

// extern crate fixed_typed_arena;
// extern crate generational_arena;
// extern crate rpool;

pub mod action;
pub mod board;
pub mod sgf;
pub mod display;

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use bit_set::BitSet;
    use graph_lib::topology::Topology;

    use board::go_state::GoState;
    use board::grid::Grid;
    use stones::groups1::GoGroup;
    use stones::grouprc::GoGroupRc;
    use stones::stone::Stone;

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
