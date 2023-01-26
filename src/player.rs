#![allow(dead_code)]

use crate::brain::Network;
use crate::payoff::PayoffMatrix;

pub struct Player {
    name: String,
    payoff: PayoffMatrix,
    brain: Network,
}

impl Player {}
