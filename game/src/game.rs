use rand;
use rand::Rng;
use std;
use std::fmt;
use crossbeam::thread;

use crate::player::Player;
use crate::quarters::Quarters;
use crate::screener::Rule;

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
        write!(f, "Game[players: {:?}, quarters: {}, current_quarter_index: {}, index_of_value: {}]", self.players, self.quarters, self.current_quarter_index, self.index_of_value)
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
        // Get the banned indicies list
        let banned_names = vec!["adj_close", "adj_factor", "adj_high", "adj_low", "adj_open", "adj_volume", "close", "high", "low", "open", "volume"];
        let mut banned_indicies = Vec::new();
        for (i, field_name) in quarters.field_names.iter().enumerate() {
            if banned_names.contains(&&field_name[0..]) {
                banned_indicies.push(i);
            }
        }
        // Make players
        let mut players = Vec::new();
        for _i in 0..num_of_players {
            players.push(Player::new_uniform_random((&l_limits, &u_limits), &banned_indicies));
        }
        Game {
            players: players,
            quarters: quarters,
            current_quarter_index: 0,
            index_of_value: 0,
            ratio: 0.4
        }
    }
    fn calculate_cheap_limits(quarters: &Quarters) -> (Vec<f64>, Vec<f64>) {
        let first_quarter = quarters.get(0).unwrap();
        let mut lower_limits = vec![std::f64::MAX; first_quarter.get(0).unwrap().len()];
        let mut upper_limits = vec![std::f64::MIN; first_quarter.get(0).unwrap().len()];
        for current_quarter in &quarters.quarters_vector {
            for ref entry in &current_quarter.quarter_vector {
                for (&field, (lower_limit, upper_limit)) in entry.iter().zip(lower_limits.iter_mut().zip(upper_limits.iter_mut())) {
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
    pub fn expensive_training_data_analysis(&self) -> Vec<Vec<f64>> {
        let mut field_accumulator: Vec<Vec<f64>> = vec![Vec::new(); self.quarters.get(0).unwrap().get(0).unwrap().len()];    // Vector of all results for all fields
        for current_quarter in &self.quarters.quarters_vector {
            for ref row in &current_quarter.quarter_vector {
                for (&field, field_store) in row.iter().zip(field_accumulator.iter_mut()) {
                    field_store.push(field);
                }
            }
        }
        for field_store in &mut field_accumulator {
            field_store.sort_by(|a, b| a.partial_cmp(b).unwrap());
        }
        //println!("{:?}", field_accumulator.iter().zip(self.quarters.field_names.iter()).collect::<Vec<_>>());
        field_accumulator
    }
    pub fn run(&mut self, mut generation_max: i64, iteration: usize) {
        let (l_limits, u_limits) = Game::calculate_cheap_limits(&self.quarters);
        let compounded_training_vectors = self.expensive_training_data_analysis();
        let quarters_len = self.quarters.len();
        for i in 0..iteration {
            for _j in 0..generation_max {
                self.perform_generation(quarters_len, DEFAULT_TOURNEY_CONST, DEFAULT_MUTATION_CONST, i);
            }
            self.perform_analytical_final_run(i);
            self.recalc_fields_used(&compounded_training_vectors);
            self.soft_reset((&l_limits, &u_limits));
            if i == 0 {
                self.ratio = 0.4;
                generation_max = 10;
            } else if i == 1 {
                self.ratio = 0.5;
            } else if i == 2 {
                self.ratio = 0.6;
            } else if i == 3 {
                self.ratio = 0.7;
            } else if i == 4 {
                self.ratio = 0.8;
            } else if i == 5 {
                self.ratio = 0.9;
            } else if i == 6 {
                self.ratio = 0.99;
            }
            println!("Run {:?} complete!", i);
            println!("{:?}", self.players[0].strategy.iter().zip(&self.quarters.field_names).filter_map(|((field, used, rule), name)| {
                if *used {
                    Some((name, rule, field))
                } else {
                    None
                }
            }).collect::<Vec<_>>());
        }
    }
    /// Run through all of the test data, and generate a new population.
    ///
    /// # Arguments
    /// * `quarter_max` - The maximum number of quarters to run through.
    /// * `k` - Constant used for tournament selection.
    /// * `mut_const` - Constant used for mutation.
    pub fn perform_generation(&mut self, quarter_max: usize, k: usize, mut_const: f64, iteration: usize) {
        while self.current_quarter_index < quarter_max - 1 {
            self.next_quarter(iteration);
        }
        self.final_quarter();
        let _normalise = self.players.iter_mut().map(|player| player.payoff_normalise()).collect::<Vec<_>>();
        let players_with_payoff = self.players.iter().fold(0, |acc, player| {
            if player.payoff != 0.0 {
                acc + 1
            } else {
                acc
            }
        });
        self.analyse_field_purchases();
        println!("Player count: {:?}, Average % Profit: {:?}", players_with_payoff, self.average_payoff());
        let mut new_population = Vec::new();
        for _player in &self.players {
            new_population.push(self.tourney_select(k).dumb_crossover(self.tourney_select(k)).lazy_mutate(mut_const));
        }
        self.players = new_population;
    }
    /// Runs through the next quarter of test data.
    fn next_quarter(&mut self, iteration: usize) {
        let quarter = self.quarters.get(self.current_quarter_index).unwrap();
        let (ratio, index_of_value) = (self.ratio, self.index_of_value);
        let player_iter = self.players.iter_mut();
        thread::scope(|s| {
            for mut player in player_iter {
                s.spawn(move |_| {
                    quarter.select_for_player(&mut player, ratio, index_of_value, iteration);
                });
            }
        }).unwrap();
        self.current_quarter_index += 1;
    }
    /// Runs through the last quarter of test data.
    fn final_quarter(&mut self) {
        println!("Starting final quarter...");
        let quarter = self.quarters.get(self.current_quarter_index).unwrap();
        let index_of_value = self.index_of_value;
        let player_iter = self.players.iter_mut();
        thread::scope(|s| {
            for mut player in player_iter {
                s.spawn(move |_| {
                    quarter.calc_payoffs(&mut player, index_of_value);
                });
            }
        }).unwrap();
        self.current_quarter_index = 0;
        println!("End of final quarter!");
    }
    /// Perform a final generation of the algorithm, purely to analyse the potential screeners
    pub fn perform_analytical_final_run(&mut self, iteration: usize) {
        while self.current_quarter_index < self.quarters.len() - 1 {
            self.next_quarter(iteration);
        }
        self.final_quarter();
        self.analyse_field_purchases();
        println!("{:?}", self.players[0].stocks_sold.iter().map(|stock| stock.stock_id.to_string()).collect::<Vec<_>>());
        // this print isn't ordered...? concerning
    }
    /// Produces some useful print data.
    fn analyse_field_purchases(&self) {
        let mut aggregate_field_counter = vec![0; self.players[0].strategy.len()];
        for player in &self.players {
            let mut player_field_counter = vec![0; player.strategy.len()];
            for stock in &player.stocks_purchased {
                for (k, (strat, used, rule)) in player.strategy.iter().enumerate() {
                    let rule_met = match rule {
                        Rule::Lt => stock.get(k) <= *strat,
                        Rule::Gt => stock.get(k) >= *strat
                    };
                    if rule_met & *used {
                        player_field_counter[k] += 1;
                    }
                }
            }
            aggregate_field_counter =  aggregate_field_counter.iter()
                                                              .zip(player_field_counter.iter())
                                                              .map(|(a, p)| a + p)
                                                              .collect();
        }
        println!("{:?}", aggregate_field_counter);
        println!("{:?}", aggregate_field_counter.iter().zip(self.players[0].strategy.iter()).filter_map(|(&counter, (_, used, _))| {
            if *used {
                Some(counter)
            } else {
                None
            }
        }).collect::<Vec<_>>());
    }
    /// Recalculate each Player's "fields_used" by using the output of analyse_field_purchases().
    pub fn recalc_fields_used(&mut self, compounded_training_vectors: &Vec<Vec<f64>>) {
        let players = &mut self.players;
        thread::scope(|s| {
            for player in players.iter_mut() {
                s.spawn(move |_| {
                    player.recalc_fields_used(&compounded_training_vectors);
                });
            }
        }).unwrap();
    }
    /// Compute the average percentage gain across the entire population.
    pub fn average_payoff(&self) -> f64 {
        self.players.iter().fold(0.0, |acc, player| {
            let field_used_symbolic_length = player.strategy.iter().fold(0.0, |acc, (_, used, _)| {
                if *used {
                    acc + 1.0
                } else {
                    acc
                }
            });
            acc + ((player.payoff * (if field_used_symbolic_length > 10.0 {field_used_symbolic_length} else {10.0} / 4.0)) / (if player.stocks_sold.len() != 0 {(player.stocks_sold.len() as f64) * (player.stocks_sold.len() as f64)} else {1.0}))
        }) / (self.players.len() as f64)
    }
    /// Soft resets the list of players.
    pub fn soft_reset(&mut self, (l_limits, u_limits): (&Vec<f64>, &Vec<f64>)) {
        for player in &mut self.players {
            player.soft_reset((l_limits, u_limits));
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
