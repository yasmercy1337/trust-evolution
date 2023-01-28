#![allow(dead_code)]
use crate::player::Player;

const NUM_ROUNDS: usize = 10;

pub struct Match {
    pub player_one: Player,
    pub player_two: Player,
}

impl Match {
    pub fn play(&mut self) -> (i16, i16) {
        // returns the scores for players 1 and 2
        for _ in 0..NUM_ROUNDS {
            self.play_round();
        }
        (
            self.player_one.calculate_score(),
            self.player_two.calculate_score(),
        )
    }

    fn play_round(&mut self) {
        let one = self.player_one.play() as usize;
        let two = self.player_two.play() as usize;

        // updating histories
        self.player_one.self_history.history.push(one);
        self.player_one.opp_history.history.push(two);
        self.player_two.self_history.history.push(two);
        self.player_two.opp_history.history.push(one);
    }
}
