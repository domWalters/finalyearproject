extern crate rand;

use rand::Rng;

pub struct Game {
    players: Vec<Player>,
    current_quarter: u64,
}

impl Game {

    fn next_quarter(&mut self) {
        // Load the new quarter
        let quarter = Quarter::load_blank();    // temp
        // Grab new stocks
        for i in 0..self.players.len() {
            quarter.select_for_player(self.players[i]);
        }
        // Go to next quarter
        self.current_quarter += 1;
    }

    fn final_quarter(&mut self) {
        // Load the final quarter
        let quarter = Quarter::load_blank();    // temp
        // Increment payoff by stock values
        for i in 0..self.players.len() {
            quarter.calc_payoff(self.players[i]);
        }
    }

    pub fn perform_generation(&mut self, quarter_max: u64) {
        // Compute payoffs
        while self.current_quarter < quarter_max {
            next_quarter();
        }
        final_quarter();
        // Select for new generation
        let mut new_population = Vec::new();
        for i in 0..self.players.len() {
            new_population.push(self.tourney_select(k).dumb_crossover(self.tourney_select(k)).mutate());
        }
        self.players = new_population;
        // End
    }

    fn tourney_select(&self, k: usize) -> &Player {
        let mut rng = rand::thread_rng();
        let candidate = self.players[rng.gen_range(0, self.players.len())];
        for i in 1..k {
            let next_candidate = self.players[rng.gen_range(0, self.players.len())];
            if next_candidate.payoff > candidate.payoff {
                candidate = next_candidate;
            }
        }
        candidate
    }

}
