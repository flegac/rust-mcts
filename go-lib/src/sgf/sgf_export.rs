use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Write;
use std::ptr::write_bytes;

use action::GoAction;
use stones::stone::Stone;

pub struct Prop {
    key: String,
    value: String,
}

impl Display for Prop {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}[{}]", self.key, self.value)
    }
}

pub struct Node {
    props: Vec<Prop>
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, ";");
        for prop in self.props.iter() {
            write!(f, "{}", prop);
        }
        fmt::Result::Ok(())
    }
}


pub struct Sequence {
    data: Vec<Node>
}

impl Display for Sequence {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "(");
        for n in self.data.iter() {
            write!(f, "{}", n);
        }
        write!(f, ")")
    }
}

pub struct SGF {}

impl SGF {
    pub fn save(board_size: usize, stone: Stone, actions: &[GoAction]) {
        if let Ok(mut file) = File::create("output.sgf") {
            file.write_all(SGF::game(board_size, stone, actions).to_string().as_bytes());
        }
    }

    fn prop(key: &str, value: &str) -> Prop {
        Prop {
            key: String::from(key),
            value: String::from(value),
        }
    }

    fn header(size: usize) -> Node {
        Node {
            props: vec![
                SGF::prop("AP", "rust-mcts"),
                SGF::prop("FF", "4"),
                SGF::prop("GM", "1"),
                SGF::prop("SZ", &size.to_string()),
                SGF::prop("KM", "5.5"),
                SGF::prop("PL", "B"),
                SGF::prop("RU", "Japanese"),
            ]
        }
    }

    fn action(stone: Stone, a: GoAction) -> Node {
        let stone_string = format!("{:?}", stone).chars().next().unwrap().to_string();

        let a_string = match a {
            GoAction::Pass => String::from("tt"),
            _ => {
                a.to_string().to_lowercase()
            }
        };

        Node {
            props: vec![
                SGF::prop(&stone_string, &a_string)
            ]
        }
    }

    pub fn game(board_size: usize, stone: Stone, actions: &[GoAction]) -> Sequence {
        let mut x = vec![SGF::header(board_size)];
        let mut side = stone;
        for &a in actions {
            x.push(SGF::action(side, a));
            side = side.switch();
        }
        Sequence {
            data: x
        }
    }
}


#[test]
fn stone_groups() {
    let main = SGF::actions(Stone::Black, &[
        GoAction::Cell(3, 2),
        GoAction::Cell(2, 2),
        GoAction::Cell(1, 1),
    ]);
    let var1 = SGF::actions(Stone::Black, &[
        GoAction::Cell(3, 2),
        GoAction::Cell(2, 2),
        GoAction::Cell(1, 1),
    ]);
    let var2 = SGF::actions(Stone::Black, &[
        GoAction::Cell(3, 2),
        GoAction::Cell(2, 2),
        GoAction::Cell(1, 1),
    ]);

    println!("{}", main);
    println!("{}", var1);
    println!("{}", var2);
}


