#![allow(dead_code)]

use crate::history::MoveHistory;
use crate::payoff::PayoffMatrix;
use crate::strategy::{Strategy, Playable};

pub struct Player {
    name: String,
    history: MoveHistory,
    payoff: PayoffMatrix,
    strategy: Strategy
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

    pub fn play(&self) -> usize {
        let mut inputs = self.payoff.data();
        inputs.extend(self.history.summary());
        self.strategy.play(inputs, self.payoff.row_options)
    }
}
