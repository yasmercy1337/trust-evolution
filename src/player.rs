#![allow(dead_code)]

use crate::history::MoveHistory;
use crate::payoff::PayoffMatrix;
use crate::strategy::{MoveOption, Playable, Strategy};

pub struct Player {
    name: String,
    history: MoveHistory,
    payoff: PayoffMatrix,
    strategy: Strategy,
}

impl Player {
    pub fn new(name: String, payoff: PayoffMatrix, strategy: Strategy) -> Self {
        Self {
            name,
            strategy,
            payoff,
            history: MoveHistory::new(),
        }
    }

    pub fn play(&mut self) -> MoveOption {
        self.strategy
            .play(self.payoff.data(), &self.history, self.calculate_score())
    }

    fn calculate_score(&self) -> i8 {
        0
    }
}
