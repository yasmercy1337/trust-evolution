mod brain;
mod history;
mod payoff;
mod player;
mod strategy;

use player::*;
use strategy::*;
use crate::payoff::PayoffMatrix;

fn main() {
    let payoff = PayoffMatrix::new(vec![
        vec![],
        vec![],
    ]);

    let player_one = Player::new(String::from("AI"),
                                 payoff.clone(),
                                 AIStrategy::new().into());

    let player_two = Player::new(String::from("Cat"),
                                 payoff.clone().transpose(),
                                 CopyCat::new().into());

    let one = player_one.play();
    let two = player_two.play();
    println!("{}{}", one, two);
}
