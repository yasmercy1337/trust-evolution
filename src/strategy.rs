#![allow(dead_code)]

use crate::brain::Brain;
use crate::history::MoveHistory;
use enum_dispatch::enum_dispatch;
use itertools::Itertools;
use rand::Rng;
use std::convert::AsRef;
use std::default::Default;
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{AsRefStr, EnumCount as EnumCountMacro, EnumIter};

#[derive(Debug, Clone, Copy)]
pub enum MoveOption {
    SPLIT,
    STEAL,
}

impl MoveOption {
    pub fn other(&self) -> Self {
        match self {
            Self::STEAL => Self::SPLIT,
            Self::SPLIT => Self::STEAL,
        }
    }
}

impl Default for MoveOption {
    fn default() -> Self {
        MoveOption::SPLIT
    }
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
#[derive(EnumCountMacro, EnumIter, AsRefStr, Debug, Clone)]
pub enum Strategy {
    AI,
    CopyCat,
    CopyKitten,
    Cooperative,
    Cheater,
    Random,
    Detective,
    Simpleton,
    Grudger,
}

#[enum_dispatch(Strategy)]
pub trait Playable {
    fn play(&mut self, payoff_data: Vec<f64>, history: &MoveHistory, score: i16) -> MoveOption;
}

#[derive(Default, Debug, Clone)]
pub struct AI {
    brain: Brain,
}

impl Playable for AI {
    fn play(&mut self, payoff_data: Vec<f64>, history: &MoveHistory, score: i16) -> MoveOption {
        let mut inputs = payoff_data
            .into_iter()
            .chain(history.summary())
            .collect_vec();
        inputs.push(score as f64);
        let out = self.brain.eval(inputs)[0];
        MoveOption::from((out > 0.0) as usize + 1)
    }
}

#[derive(Default, Debug, Clone)]
pub struct CopyCat {}

impl Playable for CopyCat {
    fn play(&mut self, _: Vec<f64>, history: &MoveHistory, _: i16) -> MoveOption {
        MoveOption::from(history.last())
    }
}

#[derive(Debug, Clone)]
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
    fn play(&mut self, _: Vec<f64>, history: &MoveHistory, _: i16) -> MoveOption {
        self.doubt += (history.last() == 2) as usize;
        if self.doubt <= self.threshold {
            return MoveOption::SPLIT;
        }
        MoveOption::from(history.last())
    }
}

#[derive(Default, Debug, Clone)]
pub struct Random {}

impl Playable for Random {
    fn play(&mut self, _: Vec<f64>, _: &MoveHistory, _: i16) -> MoveOption {
        let mut rng = rand::thread_rng();
        MoveOption::from(rng.gen_range(1..=2))
    }
}

#[derive(Default, Debug, Clone)]
pub struct Cooperative {}

impl Playable for Cooperative {
    fn play(&mut self, _: Vec<f64>, _: &MoveHistory, _: i16) -> MoveOption {
        MoveOption::SPLIT
    }
}

#[derive(Default, Debug, Clone)]
pub struct Cheater {}

impl Playable for Cheater {
    fn play(&mut self, _: Vec<f64>, _: &MoveHistory, _: i16) -> MoveOption {
        MoveOption::STEAL
    }
}

#[derive(Default, Debug, Clone)]
pub struct Detective {
    round: usize,
    cheated: bool,
}

impl Playable for Detective {
    fn play(&mut self, _: Vec<f64>, history: &MoveHistory, _: i16) -> MoveOption {
        self.round += 1;
        self.cheated = self.cheated || history.history.contains(&2);
        if self.round <= 4 {
            match self.round {
                2 => MoveOption::STEAL,
                _ => MoveOption::SPLIT,
            }
        } else if self.cheated {
            MoveOption::from(history.last())
        } else {
            MoveOption::STEAL
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct Simpleton {
    last_strat: MoveOption,
    last_score: i16,
}

impl Playable for Simpleton {
    fn play(&mut self, _: Vec<f64>, _: &MoveHistory, _: i16) -> MoveOption {
        if self.last_score >= 0 {
            self.last_strat.clone()
        } else {
            self.last_strat.other()
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct Grudger {
    cheated: bool,
}

impl Playable for Grudger {
    fn play(&mut self, _: Vec<f64>, history: &MoveHistory, _: i16) -> MoveOption {
        self.cheated = self.cheated || history.history.contains(&2);
        MoveOption::from(self.cheated as usize + 1)
    }
}
