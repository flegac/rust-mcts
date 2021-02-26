extern crate bit_set;
extern crate core;
extern crate fixed_typed_arena;
extern crate graph_lib;
extern crate itertools;
extern crate log;
extern crate mcts_lib;
extern crate proc_macro;
extern crate rpool;

pub mod stones;
pub mod action;
pub mod gostate;
pub mod board;
pub mod game;
pub mod go_display;

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use bit_set::BitSet;
    use rpool::{Pool, Poolable, PoolScaleMode};

    use board::goboard::GoBoard;
    use board::grid::Grid;
    use graph_lib::topology::Topology;
    use stones::group::GoGroup;
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
