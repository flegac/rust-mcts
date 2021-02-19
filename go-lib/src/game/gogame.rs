use std::fmt;
use std::fmt::Formatter;

use action::GoAction;
use game::gogame::Sequence::{Cons, Nil, Variation};

#[derive(Debug)]
enum Sequence {
    Nil,
    Cons(Box<Sequence>, GoAction),
    Variation(Box<Sequence>, Box<Sequence>),
}


impl fmt::Display for Sequence {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Nil => write!(f, ""),
            Cons(seq, a) => {
                write!(f, "{};{}", seq, a)
            }
            Variation(main, var) => {
                write!(f, "{}({})", main, var)
            }
        }
    }
}

impl Sequence {
    pub fn build(actions: &[GoAction]) -> Sequence {
        let mut res = Nil;
        for &a in actions {
            res = Cons(Box::new(res), a);
        }
        res
    }
}


#[test]
fn stone_groups() {
    let main = Sequence::build(&[
        GoAction::Cell(3, 2),
        GoAction::Cell(2, 2),
        GoAction::Cell(1, 1),
    ]);
    let var1 = Sequence::build(&[
        GoAction::Cell(3, 2),
        GoAction::Cell(2, 2),
        GoAction::Cell(1, 1),
    ]);
    let var2 = Sequence::build(&[
        GoAction::Cell(3, 2),
        GoAction::Cell(2, 2),
        GoAction::Cell(1, 1),
    ]);
    let var = Variation(Box::new(var1), Box::new(var2));
    let game = Variation(Box::new(main), Box::new(var));

    println!("{}", game);
}


