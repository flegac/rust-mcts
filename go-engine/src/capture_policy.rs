use go_lib::board::go_state::GoState;
use go_lib::board::group_access::GroupAccess;
use go_lib::go_rules::go_action::GoAction;
use mcts_lib::policy::policy::Policy;
use mcts_lib::rules::Rules;
use mini_nn::conv2::Conv2;

pub struct CapturePolicy<'a, A, S> {
    pub random_policy: &'a Policy<A, S>
}

impl<'a, A, S> CapturePolicy<'a, A, S> {
    fn pattern_score(&self, pattern: Conv2, state: &GoState) -> f32 {
        1.0
    }
}

impl<'a> Policy<GoAction, GoState> for CapturePolicy<'a, GoAction, GoState> {
    fn select(&self, state: &GoState) -> GoAction {
        let stone = state.current_side;
        let rand_action = self.random_policy.select(state);

        state.actions()
            .iter()
            .max_by_key(|&a| {
                match a {
                    GoAction::Pass => 0,
                    GoAction::Cell(x, y) => {
                        1
                    }
                }
            })
            .unwrap_or(&rand_action)
            .clone()
    }
}