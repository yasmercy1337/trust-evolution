#![allow(dead_code, unused_variables)]

use crate::brain::Brain;
use crate::history::MoveHistory;
use enum_dispatch::enum_dispatch;
use itertools::Itertools;
use rand::Rng;
use std::default::Default;

#[derive(Debug)]
pub enum MoveOption {
    SPLIT,
    STEAL,
}

impl From<usize> for MoveOption {
    fn from(value: usize) -> Self {
        match value {
            2 => Self::STEAL,
            _ => Self::SPLIT,
        }
    }
}

#[enum_dispatch]
pub enum Strategy {
    AI,
    CopyCat,
    CopyKitten,
    Cooperative,
    Detective,
    Simpleton,
    Cheater,
    Grudger,
    Random,
}

#[enum_dispatch(Strategy)]
pub trait Playable {
    fn play(&mut self, payoff_data: Vec<f64>, history: &MoveHistory, score: i8) -> MoveOption;
}

#[derive(Default)]
pub struct AI {
    brain: Brain,
}

impl Playable for AI {
    fn play(&mut self, payoff_data: Vec<f64>, history: &MoveHistory, score: i8) -> MoveOption {
        let mut inputs = payoff_data
            .into_iter()
            .chain(history.summary())
            .collect_vec();
        inputs.push(score as f64);
        let out = self.brain.eval(inputs)[0];
        MoveOption::from((out > 0.0) as usize + 1)
    }
}

#[derive(Default)]
pub struct CopyCat {}

impl Playable for CopyCat {
    fn play(&mut self, _: Vec<f64>, history: &MoveHistory, _: i8) -> MoveOption {
        MoveOption::from(history.last())
    }
}

pub struct CopyKitten {
    doubt: usize,
    threshold: usize,
}

impl Default for CopyKitten {
    fn default() -> Self {
        Self {
            doubt: 0,
            threshold: 1,
        }
    }
}

impl Playable for CopyKitten {
    fn play(&mut self, _: Vec<f64>, history: &MoveHistory, _: i8) -> MoveOption {
        self.doubt += (history.last() == 2) as usize;
        if self.doubt <= self.threshold {
            return MoveOption::SPLIT;
        }
        MoveOption::from(history.last())
    }
}

#[derive(Default)]
pub struct Random {}

impl Playable for Random {
    fn play(&mut self, _: Vec<f64>, _: &MoveHistory, _: i8) -> MoveOption {
        let mut rng = rand::thread_rng();
        MoveOption::from(rng.gen_range(1..=2))
    }
}

#[derive(Default)]
pub struct Cooperative {}

impl Playable for Cooperative {
    fn play(&mut self, _: Vec<f64>, _: &MoveHistory, _: i8) -> MoveOption {
        MoveOption::SPLIT
    }
}

#[derive(Default)]
pub struct Cheater {}

impl Playable for Cheater {
    fn play(&mut self, _: Vec<f64>, _: &MoveHistory, _: i8) -> MoveOption {
        MoveOption::STEAL
    }
}

#[derive(Default)]
pub struct Detective {}

impl Playable for Detective {
    fn play(&mut self, _: Vec<f64>, _: &MoveHistory, _: i8) -> MoveOption {
        todo!()
    }
}

#[derive(Default)]
pub struct Simpleton {}

impl Playable for Simpleton {
    fn play(&mut self, _: Vec<f64>, _: &MoveHistory, _: i8) -> MoveOption {
        todo!()
    }
}

#[derive(Default)]
pub struct Grudger {}

impl Playable for Grudger {
    fn play(&mut self, _: Vec<f64>, _: &MoveHistory, _: i8) -> MoveOption {
        todo!()
    }
}
