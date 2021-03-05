use crate::board::go_state::GoState;
use board::group_access::GroupAccess;
use board::stats::full_stats::FullStats;
use mcts_lib::policy::policy::Policy;
use mcts_lib::rules::Rules;
use go_rules::go_action::GoAction;

pub struct CapturePolicy<'a, A, S> {
    pub other: &'a Policy<A, S>
}

impl<'a> Policy<GoAction, GoState> for CapturePolicy<'a, GoAction, GoState> {
    fn select(&self, state: &GoState) -> GoAction {
        let stone = state.stone;

        let rand_action = self.other.select(state);
        state.actions()
            .iter()
            .max_by_key(|&a| {
                match a {
                    GoAction::Pass => 0,
                    GoAction::Cell(x, y) => {
                        let cell = state.gg.goban().cell(*x, *y);
                        let captures: usize = state.gg.adjacent_groups(cell)
                            .iter()
                            .filter(|g| g.borrow().stone == stone.switch())
                            .filter(|g| g.borrow().liberties == 1)
                            .map(|g| g.borrow().stones())
                            .sum();
                        let max_size: usize = state.gg.adjacent_groups(cell)
                            .iter()
                            .filter(|g| g.borrow().stone == stone)
                            // .filter(|g| g.borrow().liberties > 1)
                            .map(|g| g.borrow().stones())
                            .sum();
                        1 + 2*captures + max_size
                    }
                }
            })
            .unwrap_or(&rand_action)
            .clone()
    }
}