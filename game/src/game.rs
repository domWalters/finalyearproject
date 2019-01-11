extern crate rand;

use rand::Rng;
use std::fmt;

use Player;
use Quarter;

#[derive(Debug)]
pub struct Game {
    players: Vec<Player>,
    current_quarter: usize,
    index_of_value: usize
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Game[players: {:?}, current_quarter: {}, index_of_value: {:?}]", self.players, self.current_quarter, self.index_of_value)
    }
}

impl Game {

    fn next_quarter(&mut self) {
        // Load the new quarter
        let quarter = Quarter::load_blank();    // temp
        // Grab new stocks
        for i in 0..self.players.len() {
            quarter.select_for_player(&mut self.players[i]);
        }
        // Go to next quarter
        self.current_quarter += 1;
    }

    fn final_quarter(&mut self) {
        // Load the final quarter
        let quarter = Quarter::load_blank();    // temp
        // Increment payoff by stock values
        for i in 0..self.players.len() {
            quarter.calc_payoffs(&mut self.players[i], self.index_of_value);
        }
    }

    pub fn perform_generation(&mut self, quarter_max: usize, k: usize, mut_const: f64) {
        // Compute payoffs
        while self.current_quarter < quarter_max {
            self.next_quarter();
        }
        self.final_quarter();
        // Select for new generation
        let mut new_population = Vec::new();
        for _i in 0..self.players.len() {
            new_population.push(self.tourney_select(k).dumb_crossover(self.tourney_select(k)).mutate(mut_const));
        }
        self.players = new_population;
        // End
    }

    fn tourney_select(&self, k: usize) -> &Player {
        let mut rng = rand::thread_rng();
        let mut candidate = &self.players[rng.gen_range(0, self.players.len())];
        for _i in 1..k {
            let next_candidate = &self.players[rng.gen_range(0, self.players.len())];
            if next_candidate.payoff > candidate.payoff {
                candidate = next_candidate;
            }
        }
        candidate
    }

}
