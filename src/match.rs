#![allow(dead_code)]
use crate::player::Player;
use crate::strategy::MoveOption;
use rand::Rng;

const NUM_ROUNDS: usize = 10;
const COMM_ERROR: f64 = 0.2;

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

    fn communicate(option: MoveOption) -> usize {
        let mut rng = rand::thread_rng();
        if rng.gen::<f64>() <= COMM_ERROR {
            option.other() as usize + 1
        } else {
            option as usize + 1
        }
    }

    fn play_round(&mut self) {
        let one = Self::communicate(self.player_one.play());
        let two = Self::communicate(self.player_two.play());

        // updating histories
        self.player_one.self_history.history.push(one);
        self.player_one.opp_history.history.push(two);
        self.player_two.self_history.history.push(two);
        self.player_two.opp_history.history.push(one);
    }
}
