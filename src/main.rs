#![allow(unused_imports)]

mod brain;
mod history;
mod r#match;
mod payoff;
mod player;
mod strategy;
mod tournament;

use crate::payoff::PayoffMatrix;
use player::*;
use strategy::*;
use r#match::*;

fn main() {
    let payoff = PayoffMatrix::new(vec![vec![[2, 2], [-1, 3]], vec![[3, -1], [-1, -1]]]);

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
