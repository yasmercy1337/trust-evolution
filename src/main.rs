#![allow(unused_imports)]

mod brain;
mod history;
mod r#match;
mod payoff;
mod player;
mod strategy;
mod tournament;

use payoff::PayoffMatrix;
use player::*;
use r#match::*;
use strategy::*;
use tournament::Tournament;

fn get_payout() -> PayoffMatrix {
    PayoffMatrix::new(vec![vec![[2, 2], [-1, 3]], vec![[3, -1], [-1, -1]]])
}

#[allow(unused)]
fn test() {
    let payoff = get_payout();
    let player_one = Player::new(
        String::from("Training AI"),
        payoff.clone(),
        AI::default().into(),
    );

    // let player_two = Player::new(
    //     String::from("Cat"),
    //     payoff.clone().transpose(),
    //     CopyCat::default().into(),
    // );

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

fn main() {
    let population_distribution = [4; 9];
    let mut tournament = Tournament::new(get_payout(), population_distribution);
    println!("{:#?}", tournament);
    for _ in 1..population_distribution.iter().sum() {
        tournament.play();
        tournament.prune();
        println!("{:#?}", tournament);
    }
}
