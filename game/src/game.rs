use rand::Rng;
use std::fmt;

use Player;
use Quarter;
use Quarters;

static DEFAULT_TOURNEY_CONST: usize = 3;
static DEFAULT_MUTATION_CONST: f64 = 1.0;

#[derive(Debug)]
pub struct Game {
    players: Vec<Player>,
    quarters: Quarters,
    current_quarter_index: usize,
    index_of_value: usize
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Game[players: {:?}, quarters: {:?}, current_quarter_index: {}, index_of_value: {:?}]", self.players, self.quarters, self.current_quarter_index, self.index_of_value)
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
    /// Not currently implemented properly, just generates a standard random Game with players
    /// initialised between the test data element limits. Will likely need to be more sophisticated.
    pub fn new_game(quarters: Quarters, num_of_players: usize) -> Game {
        let (l_limits, u_limits) = Game::calculate_cheap_limits(&quarters);
        let mut players = Vec::new();
        for _i in 0..num_of_players {
            players.push(Player::new_uniform_random((&l_limits, &u_limits)));
        }
        Game {
            players: players,
            quarters: quarters,
            current_quarter_index: 0,
            index_of_value: 0,
        }
    }
    fn calculate_cheap_limits(quarters: &Quarters) -> (Vec<f64>, Vec<f64>) {
        let first_quarter = quarters.get(0).unwrap();
        let mut lower_limits = vec![std::f64::MAX; first_quarter.get(0).unwrap().len()];
        let mut upper_limits = vec![std::f64::MIN; first_quarter.get(0).unwrap().len()];
        for i in 0..quarters.len() {
            let current_quarter = quarters.get(i).unwrap();
            for j in 0..current_quarter.len() {
                let entry = current_quarter.get(j).unwrap();
                for k in 0..entry.len() {
                    if entry.get(k) < lower_limits[k] {
                        lower_limits[k] = entry.get(k);
                    }
                    if entry.get(k) > upper_limits[k] {
                        upper_limits[k] = entry.get(k);
                    }
                }
            }
        }
        (lower_limits, upper_limits)
    }
    /// Runs through the next quarter of test data.
    fn next_quarter(&mut self) {
        let quarter = self.quarters.get(self.current_quarter_index).unwrap();
        for i in 0..self.players.len() {
            quarter.select_for_player(&mut self.players[i]);
        }
        self.current_quarter_index += 1;
    }
    /// Runs through the last quarter of test data.
    fn final_quarter(&mut self) {
        let quarter = self.quarters.get(self.current_quarter_index).unwrap();
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
        while self.current_quarter_index < quarter_max - 1 {
            self.next_quarter();
        }
        self.final_quarter();
        println!("{:?}", self.average_payoff());
        //println!("{:?}", self.players);
        let mut new_population = Vec::new();
        for _i in 0..self.players.len() {
            new_population.push(self.tourney_select(k).dumb_crossover(self.tourney_select(k)).lazy_mutate(mut_const));
        }
        self.players = new_population;
    }

    pub fn average_payoff(&self) -> f64 {
        let mut aggregate_payoff = 0.0;
        for i in 0..self.players.len() {
            aggregate_payoff += self.players[i].payoff;
        }
        aggregate_payoff / (self.players.len() as f64)
    }
    /// Run through all of the test data, and generate a new population. Uses
    /// DEFAULT_TOURNEY_CONST and DEFAULT_MUTATION_CONST for the associated functions.
    ///
    /// # Arguments
    /// * `quarter_max` - The maximum number of quarters to run through.
    pub fn perform_generation_default(&mut self) {
        let quarters_len = self.quarters.len();
        self.perform_generation(quarters_len, DEFAULT_TOURNEY_CONST, DEFAULT_MUTATION_CONST)
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
