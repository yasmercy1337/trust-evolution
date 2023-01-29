use crate::history::MoveHistory;
use crate::payoff::PayoffMatrix;
use crate::strategy::{MoveOption, Playable, Strategy, AI};
use std::fmt::{Debug, Formatter};
use std::iter::zip;

#[derive(Clone)]
pub struct Player {
    pub name: String,
    pub self_history: MoveHistory,
    pub opp_history: MoveHistory,
    pub payoff: PayoffMatrix,
    pub strategy: Strategy,
}

impl Debug for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Player {
    pub fn new(name: String, payoff: PayoffMatrix, strategy: Strategy) -> Self {
        Self {
            name,
            strategy,
            payoff,
            self_history: MoveHistory::new(),
            opp_history: MoveHistory::new(),
        }
    }

    pub fn new_ai(payoff: PayoffMatrix) -> Self {
        Self::new("AI".to_string(), payoff, AI::default().into())
    }

    pub fn play(&mut self) -> MoveOption {
        self.strategy.play(
            self.payoff.data(),
            &self.opp_history,
            self.calculate_score(),
        )
    }

    pub fn calculate_score(&self) -> i16 {
        zip(
            self.self_history.history.iter(),
            self.opp_history.history.iter(),
        )
        .map(|(&row_choice, &col_choice)| self.payoff.get_row_payout(row_choice, col_choice))
        .sum()
    }

    pub fn mutate(&self) -> Self {
        let mut out = self.clone();
        out.strategy = out.strategy.mutate().into();
        out
    }
}
