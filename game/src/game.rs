use rand::Rng;
use std::fmt;

use Player;
use Quarters;

pub static DEFAULT_TOURNEY_CONST: usize = 3;
pub static DEFAULT_MUTATION_CONST: f64 = 0.7;

#[derive(Debug)]
pub struct Game {
    players: Vec<Player>,
    quarters: Quarters,
    current_quarter_index: usize,
    index_of_value: usize,
    ratio: f64
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {  // Overly verbose
        write!(f, "Game[players: {:?}, quarters: {:?}, current_quarter_index: {}, index_of_value: {:?}]", self.players, self.quarters, self.current_quarter_index, self.index_of_value)
    }
}

impl Game {
    /// Create a new Game object, initialised randomly. Internal game parameters set to default
    /// values.
    ///
    /// # Arguments
    /// * `num_of_players` - The number of players to create for the game.
    /// * `size_of_data` - The length of Screener/DataRecord to use.
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
            ratio: 0.5
        }
    }
    fn calculate_cheap_limits(quarters: &Quarters) -> (Vec<f64>, Vec<f64>) {
        let first_quarter = quarters.get(0).unwrap();
        let mut lower_limits = vec![std::f64::MAX; first_quarter.get(0).unwrap().len()];
        let mut upper_limits = vec![std::f64::MIN; first_quarter.get(0).unwrap().len()];
        for current_quarter in &quarters.quarters_vector {
            for ref entry in &current_quarter.quarter_vector {
                for (&field, (mut lower_limit, mut upper_limit)) in entry.record.iter().zip(lower_limits.iter_mut().zip(upper_limits.iter_mut())) {
                    if field < *lower_limit {
                        *lower_limit = field;
                    }
                    if field > *upper_limit {
                        *upper_limit = field;
                    }
                }
            }
        }
        (lower_limits, upper_limits)
    }
    /// Runs through the next quarter of test data.
    fn next_quarter(&mut self) {
        let quarter = self.quarters.get(self.current_quarter_index).unwrap();
        for mut player in self.players.iter_mut() {
            quarter.select_for_player(&mut player, self.ratio);
        }
        self.current_quarter_index += 1;
    }
    /// Runs through the last quarter of test data.
    fn final_quarter(&mut self) {
        let quarter = self.quarters.get(self.current_quarter_index).unwrap();
        for mut player in self.players.iter_mut() {
            quarter.calc_payoffs(&mut player, self.index_of_value);
        }
        self.current_quarter_index = 0;
    }
    /// Run through all of the test data, and generate a new population.
    ///
    /// # Arguments
    /// * `quarter_max` - The maximum number of quarters to run through.
    /// * `k` - Constant used for tournament selection (default: DEFAULT_TOURNEY_CONST = 3).
    /// * `mut_const` - Constant used for mutation (default: DEFAULT_MUTATION_CONST = 1).
    pub fn perform_generation(&mut self, quarter_max: usize, k: usize, mut_const: f64) {
        while self.current_quarter_index < quarter_max - 1 {
            self.next_quarter(self.ratio);
        }
        //println!("{:?}", self.players[0].stocks_purchased.iter().map(|stock| stock.stock_id.clone()).collect::<Vec<_>>());
        self.final_quarter();
        let players_with_payoff = self.players.iter().fold(0, |acc, player| {
            if player.payoff != 0.0 {
                acc + 1
            } else {
                acc
            }
        });
        self.analyse_field_purchases();
        println!("Player count: {:?}, Average % Profit: {:?}, Profit minus natural gain: {:?}", players_with_payoff, self.average_payoff(), self.average_payoff() - self.quarters.natural_gain(self.index_of_value));
        let mut new_population = Vec::new();
        for _player in &self.players {
            new_population.push(self.tourney_select(k).dumb_crossover(self.tourney_select(k)).lazy_mutate(mut_const));
        }
        self.players = new_population;
    }
    pub fn perform_analytical_final_run(&mut self) {
        while self.current_quarter_index < self.quarters.len() - 1 {
            self.next_quarter(self.ratio);
        }
        self.final_quarter();
        self.analyse_field_purchases();
    }

    pub fn analyse_field_purchases(&self) -> Vec<Vec<i64>> {
        let mut aggregate_field_counter = vec![0; self.players[0].strategy.len()];
        let mut population_field_counter = Vec::new();
        for player in &self.players {
            let mut player_field_counter = vec![0; player.strategy.len()];
            for stock in &player.stocks_purchased {
                for k in 0..player.strategy.len() {
                    if (stock.get(k) > player.strategy.get(k)) && *player.fields_used.get(k).unwrap() {
                        player_field_counter[k] += 1;
                    }
                }
            }
            aggregate_field_counter =  aggregate_field_counter.iter()
                                                              .zip(player_field_counter.iter())
                                                              .map(|(a, p)| a + p)
                                                              .collect();
            population_field_counter.push(player_field_counter);
        }
        println!("{:?}", aggregate_field_counter);
        println!("{:?}", aggregate_field_counter.iter().zip(self.players[0].fields_used.iter()).filter_map(|(&a, &f)| {
            if f {
                Some(a)
            } else {
                None
            }
        }).collect::<Vec<_>>());
        population_field_counter
    }

    pub fn recalc_fields_used(&mut self) {
        let population_field_counter = self.analyse_field_purchases();
        for (player_field_counter, player) in population_field_counter.iter().zip(self.players.iter_mut()) {
            let mut new_fields_used = Vec::new();
            for &field_count in player_field_counter {
                new_fields_used.push(field_count != 0);
            }
            player.fields_used = new_fields_used;
        }
    }

    pub fn average_payoff(&self) -> f64 {
        (100.0 * self.payoff_sum()) / (self.players.len() as f64)
    }

    pub fn payoff_sum(&self) -> f64 {
        let mut aggregate_payoff = 0.0;
        for player in &self.players {
            aggregate_payoff += player.payoff - 1.0;
        }
        aggregate_payoff
    }

    pub fn soft_reset(&mut self) {
        for mut player in &mut self.players {
            player.soft_reset();
        }
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
