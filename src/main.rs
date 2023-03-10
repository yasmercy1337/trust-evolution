#![allow(unused_imports)]

mod brain;
mod history;
mod r#match;
mod payoff;
mod player;
mod population;
mod strategy;
mod tournament;

use itertools::Itertools;
use payoff::PayoffMatrix;
use player::*;
use population::Population;
use r#match::*;
use strategy::*;
use tournament::Tournament;

fn get_payout() -> PayoffMatrix {
    PayoffMatrix::new(vec![vec![[2, 2], [-1, 3]], vec![[3, -1], [0, 0]]])
}

#[allow(unused)]
fn test() {
    let payoff = get_payout();
    let player_one = Player::new(
        String::from("Training AI"),
        payoff.clone(),
        AI::default().into(),
    );

    let player_two = Player::new(
        String::from("Random"),
        payoff.clone().transpose(),
        Random::default().into(),
    );

    let mut game = Match {
        player_one,
        player_two,
    };
    println!("{:?}", game.play());
}

#[allow(unused)]
fn tournament() {
    // AI, CopyCat, CopyKitten, Cooperative, Cheater, Random, Detective, Simpleton, Grudger,
    let population_distribution = [0, 3, 3, 3, 3, 4, 3, 3, 3];
    let mut tournament = Tournament::new(get_payout(), population_distribution);
    for _ in 0..30 {
        tournament.repopulate();
        println!("{:#?}", tournament);
    }
    //
    println!("{:#?}", tournament);
}

fn main() {
    let mut population = Population::new(get_payout(), 10);
    println!("Before training: {}", population.stats());
    population.evolve(100000);
    println!("After training: {}", population.stats());
}
