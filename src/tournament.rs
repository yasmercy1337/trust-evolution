#![allow(dead_code)]

use crate::payoff::PayoffMatrix;
use crate::player::*;
use crate::r#match::*;
use crate::strategy::Strategy;
use itertools::Itertools;
use std::collections::BTreeSet;
use std::fmt::{Debug, Formatter};
use std::iter::zip;
use strum::{EnumCount, IntoEnumIterator};

const NUM_REPLACE: usize = 5;

pub struct Tournament {
    pub players: Vec<Player>,
    payoff: PayoffMatrix,
}

impl Debug for Tournament {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.players)
    }
}

impl Tournament {
    pub fn new(payoff: PayoffMatrix, population_distribution: [usize; Strategy::COUNT]) -> Self {
        // the distribution of players with their strategies
        Self {
            players: zip(population_distribution.into_iter(), Strategy::iter())
                .map(|(a, b)| {
                    (0..a)
                        .map(|_| {
                            Player::new(b.as_ref().to_string(), payoff.clone(), b.clone())
                        })
                        .collect_vec()
                })
                .flatten()
                .collect_vec(),
            payoff,
        }
    }

    pub fn play(&mut self) -> Vec<i16> {
        // matches each player with all other plays and plays a match
        // returns the scores for the row players
        self.players
            .clone()
            .into_iter()
            .permutations(2)
            .map(|mut players| {
                (Match {
                    player_two: players.pop().unwrap(),
                    player_one: players.pop().unwrap(),
                })
                .play()
                .0
            })
            .collect_vec()
            .chunks(self.players.len() - 1)
            .map(|x| x.iter().sum())
            .collect_vec()
    }

    pub fn repopulate(&mut self) {
        let mut players = zip(self.play().into_iter(), std::mem::take(&mut self.players))
            .sorted_by(|(a, _), (b, _)| b.cmp(a)) // sorts descending
            .map(|(_, player)| player)
            .collect_vec();
        players.truncate(players.len() - NUM_REPLACE);
        players.extend_from_slice(&players.clone()[..NUM_REPLACE]);
        self.players = players;
    }
}
