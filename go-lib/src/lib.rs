extern crate bit_set;
extern crate core;
extern crate fixed_typed_arena;
extern crate itertools;
extern crate mcts_lib;
extern crate rpool;
extern crate graph_lib;

pub mod stones;
pub mod action;
pub mod gostate;
pub mod constants;
pub mod board;
pub mod game;

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use bit_set::BitSet;
    use rpool::{Pool, Poolable, PoolScaleMode};

    use board::goboard::GoBoard;
    use board::grid::{Grid, Graph};
    use constants::GOBAN_SIZE;
    use stones::group::GoGroup;
    use stones::grouprc::GoGroupRc;
    use stones::stone::Stone;
    use board::graph::Graph;

    #[test]
    fn stone_groups() {
        let goban = Grid::new(GOBAN_SIZE);
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

        assert_eq!(group.size(), 3);
    }

    #[test]
    fn board_cell_id() {
        let goban = Grid::new(GOBAN_SIZE);

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
        let board = GoBoard::new(Grid::new(GOBAN_SIZE));
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


        let gg = board.split(g);

        for ga in gg {
            println!("- {}", ga)
        }
    }
}
