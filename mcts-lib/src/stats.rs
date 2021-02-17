use std::fmt;
use std::fmt::{Display, Formatter};

const _WIN_SCORE: f32 = 1.0;
const _DRAW_SCORE: f32 = 0.5;

pub struct MctsStats<A> {
    pub action: Option<A>,
    pub explored: usize,
    pub wins: usize,
    pub draws: usize,
}

impl<A> MctsStats<A> {
    pub(crate) fn new(action: Option<A>) -> Self {
        MctsStats {
            action,
            explored: 0,
            wins: 0,
            draws: 0,
        }
    }
    pub(crate) fn score(&self, node: &MctsStats<A>) -> f32 {
        (node.wins as f32 * _WIN_SCORE + node.draws as f32 * _DRAW_SCORE) / node.explored as f32
    }
}

impl<A> fmt::Display for MctsStats<A> where
    A: Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut ss = String::new();
        match &self.action {
            None => ss.push_str("pass"),
            Some(a) => ss.push_str(&a.to_string())
        }
        write!(f, "{}",
               format!("[{}:{}/{}/{}]",
                       ss,
                       self.wins,
                       self.draws,
                       self.explored - self.wins - self.draws,
               ))
    }
}