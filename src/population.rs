#![allow(dead_code)]
// file for AI training
// stores every N generation's data in json
// uses RNG to determine opponent

use crate::payoff::PayoffMatrix;
use crate::player::Player;
use crate::r#match::Match;
use crate::strategy::{Strategy, AI};
use itertools::Itertools;
use rand::{thread_rng, Rng};
use std::iter::zip;
use strum::EnumCount;

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
                .map(|_| Player::new_ai(payoff.clone()))
                .collect_vec(),
            size,
            payoff,
        }
    }

    pub fn evolve(&mut self) {
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
    }
}
