#![allow(dead_code)]

use crate::history::MoveHistory;
use crate::payoff::PayoffMatrix;
use crate::player::Player;
use crate::r#match::Match;
use crate::strategy::{Strategy, AI};
use itertools::Itertools;
use rand::{thread_rng, Rng};
use std::iter::zip;
use strum::{EnumCount, IntoEnumIterator};

#[derive(Debug)]
pub struct Population {
    size: usize,
    population: Vec<Player>,
    payoff: PayoffMatrix,
    best_prev: Player,
}

impl Population {
    pub fn new(payoff: PayoffMatrix, size: usize) -> Self {
        Self {
            best_prev: Player::new_ai(payoff.clone()),
            population: (0..size)
                .map(|_| Player::new_ai(payoff.clone()).mutate())
                .collect_vec(),
            size,
            payoff,
        }
    }

    pub fn evolve(&mut self, num_generations: usize) {
        for _ in 0..num_generations {
            self.iterate_generation()
        }
    }

    pub fn stats(&self) -> String {
        let mut all_self_history = MoveHistory::new();
        let mut all_other_history = MoveHistory::new();

        for strategy in Strategy::iter() {
            let mut game = Match {
                player_one: self.best_prev.clone(),
                player_two: Player::new("".to_string(), self.payoff.clone(), strategy),
            };
            game.play();

            all_self_history
                .history
                .extend(game.player_one.self_history.history.iter());
            all_other_history
                .history
                .extend(game.player_one.opp_history.history.iter());
        }

        return format!(
            "Score: {}, Game Summary: {:?}",
            all_self_history.score(&all_other_history, &self.best_prev.payoff),
            all_self_history.summary()
        );
    }

    pub fn score(&self) -> i16 {
        Strategy::iter()
            .map(|strategy| {
                Match {
                    player_one: self.best_prev.clone(),
                    player_two: Player::new("".to_string(), self.payoff.clone(), strategy),
                }
                .play()
                .0
            })
            .sum()
    }

    pub fn iterate_generation(&mut self) {
        let mut rng = thread_rng();
        let opponent = Player::new(
            "Opponent".to_string(),
            self.payoff.clone(),
            rng.gen_range(1..Strategy::COUNT).into(),
        );

        let mut best = zip(
            self.population.clone(),
            std::mem::take(&mut self.population)
                .into_iter()
                .map(|player_one| {
                    Match {
                        player_one,
                        player_two: opponent.clone(),
                    }
                    .play()
                    .0
                }),
        )
        .max_by_key(|&(_, score)| score)
        .unwrap()
        .0;

        // TODO: do some logging here idk

        // recreate next population
        self.population = (1..self.size).map(|_| best.mutate()).collect_vec();
        std::mem::swap(&mut best, &mut self.best_prev);
        self.population.push(best);
        // do some sampling --> get out of local minima
    }
}
