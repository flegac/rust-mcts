use crate::state::{GameResult, State};

#[derive(Debug, Eq, PartialEq)]
pub struct GoState {
    explored: usize,
    evaluation: usize,
}

#[derive(Debug)]
pub struct GoAction {
    pub x: usize,
    pub y: usize,
}


impl State<GoAction> for GoState {
    fn new() -> GoState {
        GoState {
            explored: 0,
            evaluation: 50,
        }
    }

    fn final_value(self) -> Option<GameResult> {
        unimplemented!()
    }

    fn eval(self) -> usize {
        self.evaluation
    }

    fn actions(&self) -> Vec<GoAction> {
        vec![GoAction { x: 0, y: 0 }]
    }

    fn next(&self, action: &GoAction) -> Self {
        GoState::new()
    }
}
