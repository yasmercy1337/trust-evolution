use crate::payoff::PayoffMatrix;
use itertools::Itertools;
use std::collections::HashMap;
use std::iter::zip;

#[derive(Default, Debug, Clone)]
pub struct MoveHistory {
    pub history: Vec<usize>,
}

impl MoveHistory {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn summary(&self) -> Vec<f64> {
        vec![self.mean(), self.median(), self.mode(), self.last() as f64]
    }

    pub fn last(&self) -> usize {
        *self.history.last().unwrap_or(&0)
    }

    pub fn score(&self, other: &MoveHistory, payoff: &PayoffMatrix) -> i16 {
        zip(self.history.iter(), other.history.iter())
            .map(|(&row_choice, &col_choice)| payoff.get_row_payout(row_choice, col_choice))
            .sum()
    }

    fn median(&self) -> f64 {
        if self.history.is_empty() {
            return 0.0;
        }
        *self.history.iter().sorted().collect_vec()[self.history.len() / 2] as f64
    }

    fn mean(&self) -> f64 {
        if self.history.is_empty() {
            return 0.0;
        }
        self.history.iter().sum::<usize>() as f64 / self.history.len() as f64
    }

    fn mode(&self) -> f64 {
        if self.history.is_empty() {
            return 0.0;
        }
        let mut map = HashMap::new();
        for item in &self.history {
            *map.entry(item).or_insert(0) += 1;
        }
        **map.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap().0 as f64
    }
}
