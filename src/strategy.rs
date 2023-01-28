#![allow(dead_code, unused_variables)]

use crate::brain::Brain;
use enum_dispatch::enum_dispatch;

const DEFAULT_MODE: usize = 1;
const MODES: [(usize, usize); 3] = [(5, 5), (10, 10), (20, 20)];

#[enum_dispatch]
pub enum Strategy {
    AIStrategy,
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
    fn play(&self, inputs: Vec<f64>, num_options: usize) -> usize;
}

pub struct AIStrategy {
    brain: Brain,
}

impl AIStrategy {
    pub fn new() -> Self {
        let (depth, width) = MODES[DEFAULT_MODE];
        Self {
            brain: Brain::new(depth, width, 1),
        }
    }
}

impl Playable for AIStrategy {
    fn play(&self, inputs: Vec<f64>, num_options: usize) -> usize {
        let out = self.brain.eval(inputs)[0];
        out as usize % num_options
    }
}

pub struct CopyCat {

}

impl CopyCat {
    pub fn new() -> Self {
        Self {}
    }
}

impl Playable for CopyCat {
    fn play(&self, inputs: Vec<f64>, num_options: usize) -> usize {
        todo!()
    }
}

pub struct CopyKitten {

}

impl CopyKitten {
    pub fn new() -> Self {
        Self {}
    }
}

impl Playable for CopyKitten {
    fn play(&self, inputs: Vec<f64>, num_options: usize) -> usize {
        todo!()
    }
}

pub struct Random {

}

impl Random {
    pub fn new() -> Self {
        Self {}
    }
}

impl Playable for Random {
    fn play(&self, inputs: Vec<f64>, num_options: usize) -> usize {
        todo!()
    }
}

pub struct Cooperative {

}

impl Cooperative {
    pub fn new() -> Self {
        Self {}
    }
}

impl Playable for Cooperative {
    fn play(&self, inputs: Vec<f64>, num_options: usize) -> usize {
        todo!()
    }
}

pub struct Detective {

}

impl Detective {
    pub fn new() -> Self {
        Self {}
    }
}

impl Playable for Detective {
    fn play(&self, inputs: Vec<f64>, num_options: usize) -> usize {
        todo!()
    }
}

pub struct Simpleton {

}

impl Simpleton {
    pub fn new() -> Self {
        Self {}
    }
}

impl Playable for Simpleton {
    fn play(&self, inputs: Vec<f64>, num_options: usize) -> usize {
        todo!()
    }
}

pub struct Cheater {

}

impl Cheater {
    pub fn new() -> Self {
        Self {}
    }
}

impl Playable for Cheater {
    fn play(&self, inputs: Vec<f64>, num_options: usize) -> usize {
        todo!()
    }
}

pub struct Grudger {

}

impl Grudger {
    pub fn new() -> Self {
        Self {}
    }
}

impl Playable for Grudger {
    fn play(&self, inputs: Vec<f64>, num_options: usize) -> usize {
        todo!()
    }
}
