use std::fmt;
use std::fmt::Formatter;

use stones::group::GoGroup;
use stones::stone::Stone;
use action::GoAction;
use display::display::GoDisplay;
use display::goshow::GoShow;

impl fmt::Display for Stone {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Stone::Black => "X",
            Stone::White => "O",
            Stone::None => "."
        })
    }
}


impl fmt::Display for GoGroup {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut res = String::new();
        res.push_str(&format!("{} #{}:", self.stone, self.stones()));
        for cell in self.cells.iter() {
            res.push_str(" ");
            res.push_str(format!("{} ", cell).as_str());
        }
        res.push_str("}");
        write!(f, "{}", res)
    }
}

impl fmt::Display for GoAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", GoDisplay::action(self))
    }
}
