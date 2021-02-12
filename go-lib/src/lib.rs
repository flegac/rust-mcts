extern crate bit_set;
extern crate core;
extern crate itertools;
extern crate mcts_lib;

pub mod stone;
pub mod action;
pub mod gostate;
pub mod board;
pub mod stone_group;
pub mod constants;

#[cfg(test)]
mod tests {
    use board::GoBoard;
    use stone_group::StoneGroup;

    #[test]
    fn stone_groups() {
        let board = GoBoard::new();


        let mut empty = StoneGroup::new(None);
        empty.cells.insert(board.cell(0, 0));
        empty.cells.insert(board.cell(0, 3));
        empty.cells.insert(board.cell(3, 0));

        assert_eq!(empty.cells.len(), 3);


    }

    #[test]
    fn board_cell_id() {
        let board = GoBoard::new();

        for c in board.cells() {
            let (x, y) = board.xy(c);
            let c2 = board.cell(x, y);
            let (x2, y2) = board.xy(c2);

            assert_eq!(c, c2);
            assert_eq!(x, x2);
            assert_eq!(y, y2);
        }
    }


    #[test]
    fn test2() {
        use std::collections::HashSet;
// Type inference lets us omit an explicit type signature (which
// would be `HashSet<String>` in this example).
        let mut books = HashSet::new();

// Add some books.
        books.insert("A Dance With Dragons".to_string());
        books.insert("To Kill a Mockingbird".to_string());
        books.insert("The Odyssey".to_string());
        books.insert("The Great Gatsby".to_string());

// Check for a specific one.
        if !books.contains("The Winds of Winter") {
            println!("We have {} books, but The Winds of Winter ain't one.",
                     books.len());
        }

// Remove a book.
        books.remove("The Odyssey");

// Iterate over everything.
        for book in &books {
            println!("{}", book);
        }
    }
}
