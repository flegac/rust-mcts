use board::group_access::GroupAccess;
use board::stats::full_stats::FullStats;
use go_rules::go_action::GoAction;
use mcts_lib::policy::policy::Policy;
use mcts_lib::rules::Rules;

use crate::board::go_state::GoState;

pub struct CapturePolicy<'a, A, S> {
    pub random_policy: &'a Policy<A, S>
}

impl<'a, A, S> CapturePolicy<'a, A, S> {
    fn pattern_score(&self, state: &GoState) -> f32 {
        1.0
    }
}

impl<'a> Policy<GoAction, GoState> for CapturePolicy<'a, GoAction, GoState> {
    fn select(&self, state: &GoState) -> GoAction {
        let stone = state.stone;
        let rand_action = self.random_policy.select(state);

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
                        1 + 2 * captures + max_size
                    }
                }
            })
            .unwrap_or(&rand_action)
            .clone()
    }
}