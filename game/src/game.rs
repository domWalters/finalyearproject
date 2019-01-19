use rand::Rng;
use std::fmt;

use Player;
use Quarter;

static DEFAULT_TOURNEY_CONST: usize = 3;
static DEFAULT_MUTATION_CONST: f64 = 1.0;

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
    /// Create a new Game object, initialised randomly. Internal game parameters set to default
    /// values.
    ///
    /// # Arguments
    /// * `num_of_players` - The number of players to create for the game.
    /// * `size_of_data` - The length of DataSlice to use.
    ///
    /// # Remarks
    /// Not currently implemented properly, just generates a very standard random Game. WIll need
    /// redoing after test data is procured.
    pub fn new_game(num_of_players: usize, size_of_data: usize) -> Game {
        let mut l_limits = Vec::new();
        let mut r_limits = Vec::new();
        for i in 0..size_of_data {
            l_limits.push(i as f64);
            r_limits.push((i + 100) as f64);
        }
        let mut players = Vec::new();
        for _i in 0..num_of_players {
            players.push(Player::new_uniform_random((&l_limits, &r_limits)));
        }
        Game {
            players: players,
            current_quarter: 0,
            index_of_value: 0,
        }
    }
    /// Runs through the next quarter of test data.
    fn next_quarter(&mut self) {
        let quarter = Quarter::load_blank();    // temp
        for i in 0..self.players.len() {
            quarter.select_for_player(&mut self.players[i]);
        }
        self.current_quarter += 1;
    }
    /// Runs through the last quarter of test data.
    fn final_quarter(&mut self) {
        let quarter = Quarter::load_blank();    // temp
        for i in 0..self.players.len() {
            quarter.calc_payoffs(&mut self.players[i], self.index_of_value);
        }
    }
    /// Run through all of the test data, and generate a new population.
    ///
    /// # Arguments
    /// * `quarter_max` - The maximum number of quarters to run through.
    /// * `k` - Constant used for tournament selection (default: DEFAULT_TOURNEY_CONST = 3).
    /// * `mut_const` - Constant used for mutation (default: DEFAULT_MUTATION_CONST = 1).
    pub fn perform_generation(&mut self, quarter_max: usize, k: usize, mut_const: f64) {
        while self.current_quarter < quarter_max {
            self.next_quarter();
        }
        self.final_quarter();
        let mut new_population = Vec::new();
        for _i in 0..self.players.len() {
            new_population.push(self.tourney_select(k).dumb_crossover(self.tourney_select(k)).mutate(mut_const));
        }
        self.players = new_population;
    }
    /// Run through all of the test data, and generate a new population. Uses
    /// DEFAULT_TOURNEY_CONST and DEFAULT_MUTATION_CONST for the associated functions.
    ///
    /// # Arguments
    /// * `quarter_max` - The maximum number of quarters to run through.
    pub fn perform_generation_default(&mut self, quarter_max: usize) {
        self.perform_generation(quarter_max, DEFAULT_TOURNEY_CONST, DEFAULT_MUTATION_CONST)
    }
    /// Perform a tournament selection of size k within the current list of Players. The fitness
    /// function is the current payoff value of each player.
    ///
    /// # Arguments
    /// * `k` - Constant used for tournament selection (default: DEFAULT_TOURNEY_CONST = 3).
    ///
    /// # Remarks
    /// This will fail at runtime if called with k = 0.
    fn tourney_select(&self, k: usize) -> &Player {
        let mut rng = rand::thread_rng();
        let mut candidate = &self.players[rng.gen_range(0, self.players.len())];
        if k == 0 {
            panic!("Tournament Selection with k = 0 occurred. Unrecoverable error.");
        } else {
            for _i in 1..k {
                let next_candidate = &self.players[rng.gen_range(0, self.players.len())];
                if next_candidate.payoff > candidate.payoff {
                    candidate = next_candidate;
                }
            }
            candidate
        }
    }
}
