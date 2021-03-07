use std::{env, fs};
use std::borrow::Borrow;
use std::fs::File;
use std::io::Write;
use std::ops::Deref;
use std::path::Path;

use constants::{BENCH, GOBAN_SIZE, SIM_FACTOR};
use go_lib::board::go_state::GoState;
use go_lib::display::display::GoDisplay;
use go_lib::display::goshow::GoShow;
use go_lib::go_rules::go_action::GoAction;
use go_lib::go_rules::go_rules::GoRules;
use mcts_lib::explorator::Explorer;
use mcts_lib::mcts::Mcts;
use mcts_lib::policy::policy::Policy;
use mcts_lib::policy::random_policy::RandomPolicy;
use mcts_lib::policy::score::Score;
use mcts_lib::policy::win_score::WinScore;
use mcts_lib::rules::{Action, Rules};
use mcts_lib::sim_result::SimResult;
use rust_tools::bench::Bench;
use rust_tools::screen::layout::layout::Layout;

fn load_sgf(filename: &Path) -> Result<String, String> {
    match fs::read_to_string(filename) {
        Ok(content) => {
            Ok(content)
        }
        Err(_) => Err(String::from("File not found !"))
    }
}

pub fn reload_sgf() {
    if let Ok(mut path) = env::current_dir() {
        path.push("output.sgf");
        println!("path: {:?}", path.as_path());
        if let Ok(game) = load_sgf(&path) {
            println!("game: {}", game);
        }
    }
}

pub fn save_sgf(state: &GoState) {
    if let Ok(mut file) = File::create("full_game.sgf") {
        file.write_all(
            GoDisplay::sgf(state)
                .to_string().as_bytes()
        );
    }
}


pub fn show_best_variant(explorator: &mut Explorer<GoAction, GoState>) {
    explorator.mcts_mut().state_mut().update_score();
    let board = explorator.mcts().state();
    GoDisplay::board(board).show();
    log::info!("root max depth: {}", explorator.mcts().borrow().root().max_depth());
}
