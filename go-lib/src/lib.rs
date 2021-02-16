extern crate bit_set;
extern crate core;
extern crate itertools;
extern crate mcts_lib;

pub mod stones;
pub mod action;
pub mod gostate;
pub mod constants;
pub mod board;

#[cfg(test)]
mod tests {
    use bit_set::BitSet;

    use board::goban::Goban;
    use board::goboard::GoBoard;
    use constants::GOBAN_SIZE;
    use stones::group::GoGroup;
    use stones::grouprc::GoGroupRc;
    use stones::stone::Stone;

    #[test]
    fn stone_groups() {
        let goban = Goban::new(GOBAN_SIZE);


        let mut cells = BitSet::new();
        for cell in &[
            goban.cell(0, 0),
            goban.cell(0, 3),
            goban.cell(3, 0)
        ] {
            cells.insert(*cell);
        }

        let group = GoGroup::new(Stone::None, cells);

        assert_eq!(group.size(), 3);
    }

    #[test]
    fn board_cell_id() {
        let goban = Goban::new(GOBAN_SIZE);

        for c in goban.cells.iter() {
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
        let board = GoBoard::new(Goban::new(GOBAN_SIZE));
        let test1 = |c| {
            let (x, y) = board.goban.xy(c);
            x == 0
        };
        let test2 = |c| {
            let (x, y) = board.goban.xy(c);
            x == 2
        };
        let mut cells1 = board.goban.flood(board.goban.cell(0, 0), &test1);
        cells1.union_with(&board.goban.flood(board.goban.cell(2, 0), &test2));
        let mut g = GoGroupRc::new(Stone::White, cells1);
        println!("big group: {}", g);


        let gg = board.split(g);

        for ga in gg {
            println!("- {}", ga)
        }
    }
}
