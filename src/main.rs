mod brain;
mod history;
mod payoff;
mod player;
mod strategy;

use crate::payoff::PayoffMatrix;
use player::*;
use strategy::*;

fn main() {
    let payoff = PayoffMatrix::new(vec![vec![], vec![]]);

    let mut player_one = Player::new(String::from("AI"), payoff.clone(), AI::default().into());

    // let mut player_two = Player::new(
    //     String::from("Cat"),
    //     payoff.clone().transpose(),
    //     CopyCat::default().into(),
    // );

    let mut player_two = Player::new(
        String::from("Random"),
        payoff.clone().transpose(),
        Random::default().into(),
    );

    let one = player_one.play();
    let two = player_two.play();
    println!("{:?} {:?}", one, two);
}
