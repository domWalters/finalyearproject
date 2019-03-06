use rand::Rng;
use std::{env::current_dir, error::Error, fmt, fs::File, io::{Write, Read}};
use crossbeam::thread;

use crate::data_trait::DataTrait;
use crate::player::Player;
use crate::quarters::Quarters;
use crate::screener::{Screener, Rule};

pub static DEFAULT_TOURNEY_CONST: usize = 3;
pub static DEFAULT_MUTATION_CONST: f64 = 0.7;

#[derive(Debug)]
pub struct Game<T: DataTrait> {
    players: Vec<Player<T>>,
    quarters_initial: Quarters<f64>,
    quarters_actual: Quarters<T>,
    current_quarter_index: usize,
    index_of_value: usize,
    pub ratio: f64
}

impl<T: DataTrait> fmt::Display for Game<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {  // Overly verbose
        write!(f, "Game[players: {:?}, quarters_initial: {:?}, quarters_actual: {:?}, current_quarter_index: {}, index_of_value: {}]", self.players, self.quarters_initial, self.quarters_actual, self.current_quarter_index, self.index_of_value)
    }
}

impl<T: DataTrait> Game<T> {
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
    pub fn new_game(quarters_initial: Quarters<f64>, num_of_players: usize, percentile_gap: usize) -> Game<usize> {
        // Get the banned indicies list
        let banned_names = vec!["adj_close", "adj_factor", "adj_high", "adj_low", "adj_open", "adj_volume", "close", "high", "low", "open", "volume"];
        let mut banned_indicies = Vec::new();
        for (i, field_name) in quarters_initial.field_names.iter().enumerate() {
            if banned_names.contains(&&field_name[0..]) {
                banned_indicies.push(i);
            }
        }
        // Create the actual quarters, and it's limits.
        let quarters_actual = quarters_initial.create_percentile_quarters(percentile_gap, quarters_initial.expensive_training_data_analysis());
        let (l_limits, u_limits) = Game::calculate_cheap_limits(&quarters_actual);
        // Make players
        let mut players = Vec::new();
        for _i in 0..num_of_players {
            players.push(Player::new_uniform_random((&l_limits, &u_limits), &banned_indicies, percentile_gap));
        }
        Game {
            players: players,
            quarters_initial: quarters_initial,
            quarters_actual: quarters_actual,
            current_quarter_index: 0,
            index_of_value: 0,
            ratio: 1.0
        }
    }
    fn calculate_cheap_limits(quarters: &Quarters<T>) -> (Vec<T>, Vec<T>) {
        let first_quarter = quarters.get(0).unwrap();
        let mut lower_limits = vec![T::max_value(); first_quarter.get(0).unwrap().len()];
        let mut upper_limits = vec![T::min_value(); first_quarter.get(0).unwrap().len()];
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
    /// Runs the algorithm.
    ///
    /// # Arguments
    /// * `generation_max` - The max number of generations to execute each time.
    /// * `iteration`- The number of iterations over the whole algorithm that should be performed.
    pub fn run(&mut self, generation_max: i64, iteration: usize, percentile_gap: usize, file_name: String) {
        let (l_limits, u_limits) = Game::calculate_cheap_limits(&self.quarters_actual);
        let compounded_training_vectors = self.quarters_actual.expensive_training_data_analysis();
        let quarters_len = self.quarters_actual.len();
        for i in 0..iteration {
            for _j in 0..generation_max {
                self.perform_generation(quarters_len, DEFAULT_TOURNEY_CONST, DEFAULT_MUTATION_CONST, i, percentile_gap);
            }
            self.perform_analytical_final_run(i);
            println!("Run {} complete!", i);
            self.print_best();
            //self.recalc_fields_used(&compounded_training_vectors);
            if i != iteration - 1 {
                self.soft_reset((&l_limits, &u_limits));
            }
        }
        self.save(file_name);
    }
    /// Run through the training data, and generate a new population.
    ///
    /// # Arguments
    /// * `quarter_max` - The maximum number of quarters to run through.
    /// * `k` - Constant used for tournament selection.
    /// * `mut_const` - Constant used for mutation.
    /// * `iteration` - The number of the current iteration.
    pub fn perform_generation(&mut self, quarter_max: usize, k: usize, mut_const: f64, iteration: usize, percentile_gap: usize) {
        while self.current_quarter_index < quarter_max - 1 {
            self.next_quarter(iteration);
        }
        self.final_quarter(iteration);
        let players_with_payoff = self.players.iter().fold(0, |acc, player| if player.payoff() != 0.0 {acc + 1} else {acc});
        self.analyse_field_purchases();
        println!("Player Count: {}, Average Profit: {:.3}%", players_with_payoff, self.average_payoff());
        self.print_best();
        let mut new_population = Vec::new();
        for _player in &self.players {
            let mut new_player = self.tourney_select(k).dumb_crossover(self.tourney_select(k), percentile_gap).lazy_mutate(mut_const, percentile_gap);
            while self.contains_species(&new_population, &new_player) {//new_player.strategy.iter().fold(0, |acc, (_, used, _)| if *used {acc + 1} else {acc}) < 1 {
                new_player = self.tourney_select(k).dumb_crossover(self.tourney_select(k), percentile_gap).lazy_mutate(mut_const, percentile_gap);
            }
            new_population.push(new_player);
        }
        self.players = new_population;
    }

    fn contains_species(&self, new_population: &Vec<Player<T>>, new_player: &Player<T>) -> bool {
        for player in new_population {
            if player.is_same_species_as(new_player) {
                return true;
            }
        }
        false
    }

    /// Runs through the next quarter of test data.
    ///
    /// # Arguments
    /// * `iteration` - The number of the current iteration.
    fn next_quarter(&mut self, iteration: usize) {
        let quarter = self.quarters_actual.get(self.current_quarter_index).unwrap();
        let float_quarter = self.quarters_initial.get(self.current_quarter_index).unwrap();
        let (ratio, index_of_value) = (self.ratio, self.index_of_value);
        let player_iter = self.players.iter_mut();
        thread::scope(|s| {
            for mut player in player_iter {
                s.spawn(move |_| {
                    quarter.select_for_player(&float_quarter, &mut player, ratio, index_of_value, iteration);
                });
            }
        }).unwrap();
        self.current_quarter_index += 1;
    }
    /// Runs through the final quarter of test data.
    ///
    /// # Arguments
    /// * `iteration` - The number of the current iteration.
    fn final_quarter(&mut self, iteration: usize) {
        println!("Starting final quarter...");
        self.next_quarter(iteration);
        self.current_quarter_index = 0;
        println!("End of final quarter!");
    }
    /// Perform a final generation of the algorithm, purely to analyse the potential screeners
    ///
    /// # Arguments
    /// * `iteration` - The number of the current iteration.
    pub fn perform_analytical_final_run(&mut self, iteration: usize) {
        while self.current_quarter_index < self.quarters_actual.len() - 1 {
            self.next_quarter(iteration);
        }
        self.final_quarter(iteration);
        self.analyse_field_purchases();
        println!("{:?}", self.players[0].stocks_sold.iter().map(|(_, _, stock)| stock.stock_id.to_string()).collect::<Vec<_>>());
        println!("{:?} - {:?}", self.players[0].spend_return, self.players[0].spend);
    }
    /// Produces print data on which fields are being bought.
    fn analyse_field_purchases(&self) {
        let mut aggregate_field_counter = vec![0; self.players[0].strategy.len()];
        for player in &self.players {
            let mut player_field_counter = vec![0; player.strategy.len()];
            for (_, _, stock) in &player.stocks_sold {
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
        println!("All purchases: {:?}", aggregate_field_counter);
    }
    /// Recalculate each Player's "fields_used" by using the output of analyse_field_purchases().
    pub fn recalc_fields_used(&mut self, compounded_training_vectors: &Vec<Vec<T>>) {
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
        let years = self.quarters_actual.years();
        let filtered_players = self.players.iter().filter(|player| player.spend_return > player.spend).collect::<Vec<_>>();
        filtered_players.iter().fold(0.0, |acc, player| acc + player.payoff_per_year(years)) / (filtered_players.len() as f64)
    }
    ///
    pub fn print_best(&self) {
        let years = self.quarters_actual.years();
        let filtered_players = self.players.iter().filter(|player| player.spend_return > player.spend).collect::<Vec<_>>();
        let mut filtered_players_iter = filtered_players.iter();
        match filtered_players_iter.next() {
            Some(player) => {
                let init_screen = &player.strategy;
                let init_acc = player.payoff_per_year(years);
                let (best_payoff, best_screener) = filtered_players_iter.fold((init_acc, init_screen), |(acc, acc_screen), player| {
                    let new_acc = player.payoff_per_year(years);
                    if new_acc > acc {
                        (new_acc, &player.strategy)
                    } else {
                        (acc, acc_screen)
                    }
                });
                println!("Best Payoff: {:.3}%, with Screener: {:?}", best_payoff, best_screener.format_screen(&self.quarters_actual));
            },
            None => {
                println!("Best Payoff: Didn't exist.");
            }
        }
    }
    /// Calls each players soft reset function.
    pub fn soft_reset(&mut self, (l_limits, u_limits): (&Vec<T>, &Vec<T>)) {
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
    fn tourney_select(&self, k: usize) -> &Player<T> {
        let mut rng = rand::thread_rng();
        let mut candidate = &self.players[rng.gen_range(0, self.players.len())];
        if k == 0 {
            panic!("Tournament Selection with k = 0 occurred. Unrecoverable error.");
        } else {
            for _i in 1..k {
                let next_candidate = &self.players[rng.gen_range(0, self.players.len())];
                if next_candidate.payoff_transform() > candidate.payoff_transform() {
                    candidate = next_candidate;
                }
            }
            candidate
        }
    }
    /// Save the current set of strategies in a human readable format
    pub fn save(&self, file_name: String) {
        let mut path = current_dir().unwrap();
        path.pop(); path.push(file_name);
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create file {:?}: {}", path, why.description()),
            Ok(file) => file,
        };
        let years = self.quarters_actual.years();
        for player in &self.players {
            let output_string = format!["Payoff: {:.3}%, Screen: {:?}\n", player.payoff_per_year(years), player.format_screen(&self.quarters_actual)];
            match file.write_all(output_string.as_bytes()) {
                Err(why) => panic!("couldn't write to file {:?}: {}", path, why.description()),
                Ok(_) => println!("successfully wrote to {:?}", path)
            }
        }
    }

    pub fn read(&mut self, file_name: String) {
        // Create a path to the desired file
        let mut path = current_dir().unwrap();
        path.pop(); path.push(file_name);
        let display = path.display();

        // Open the path in read-only mode, returns `io::Result<File>`
        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why.description()),
            Ok(file) => file,
        };

        // Read the file contents into a string, returns `io::Result<usize>`
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", display, why.description()),
            Ok(_) => {
                let mut screener_vector = Vec::new();
                // looks like [(name, rule, value), (name, rule, value), (name, rule, value), .., (name, rule, value)]
                let s = s[2..(s.len() - 3)].to_string();    // remove the starting [( and ending )]
                let split: Vec<&str> = s.split("), (").collect();
                let mut last_name_checked = None;
                for screen_rule in split {
                    let string_elements: Vec<&str> = screen_rule.split(", ").collect();  // this is a vector [name, rule, value]
                    'a: for (i, name) in self.quarters_actual.field_names.iter().enumerate() {
                        match last_name_checked {
                            None => {
                                let string_name = &string_elements[0][1..(string_elements[0].len()-1)];
                                let string_rule = string_elements[1];
                                let string_value = string_elements[2];
                                last_name_checked = Some(i);
                                if name == string_name {
                                    screener_vector.push((T::from(string_value.parse::<f64>().unwrap()).unwrap(), true, if string_rule.contains("Lt") {Rule::Lt} else {Rule::Gt}));
                                    break 'a;
                                } else {
                                    screener_vector.push((T::zero(), false, Rule::Gt));
                                }
                            }
                            Some(j) => {
                                if i > j {
                                    let string_name = &string_elements[0][1..(string_elements[0].len()-1)];
                                    let string_rule = string_elements[1];
                                    let string_value = string_elements[2];
                                    last_name_checked = Some(i);
                                    if name == string_name {
                                        screener_vector.push((T::from(string_value.parse::<f64>().unwrap()).unwrap(), true, if string_rule.contains("Lt") {Rule::Lt} else {Rule::Gt}));
                                        break 'a;
                                    } else {
                                        screener_vector.push((T::zero(), false, Rule::Gt));
                                    }
                                } else {
                                    continue 'a;
                                }
                            }
                        }
                    }
                    // find out which position it should go in
                    // fill the screener with garbage until that point
                }
                // Fill after the last rule until full
                for (i, name) in self.quarters_actual.field_names.iter().enumerate() {
                    match last_name_checked {
                        None => {
                            screener_vector.push((T::zero(), false, Rule::Gt));
                        }
                        Some(j) => {
                            if i > j {
                                screener_vector.push((T::zero(), false, Rule::Gt));
                            } else {
                                continue;
                            }
                        }
                    }
                }
                self.players = vec![Player::new_player(Screener {
                    screen: screener_vector
                })];
            },
        }
    }

}
