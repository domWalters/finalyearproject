use crossbeam::thread;
use rand::Rng;
use std::{fmt, slice::Iter, slice::IterMut};

use crate::data_trait::DataTrait;
use crate::player::Player;
use crate::quarter::Quarter;
use crate::quarters::Quarters;

#[derive(Debug)]
pub struct Population<T: DataTrait> {
    pub players: Vec<Player<T>>
}

impl<T: DataTrait> fmt::Display for Population<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Game[players: {:?}]", self.players)
    }
}

impl<T: DataTrait> Population<T> {
    /// Runs through the next quarter of test data.
    ///
    /// # Arguments
    /// * `iteration` - The number of the current iteration.
    pub fn next_quarter(&mut self, quarter: &Quarter<T>, float_quarter: &Quarter<f64>, index_of_value: usize, iteration: usize, current_quarter_index: &mut usize) {
        let player_iter = self.iter_mut();
        thread::scope(|s| {
            for mut player in player_iter {
                s.spawn(move |_| {
                    quarter.select_for_player(&float_quarter, &mut player, index_of_value, iteration);
                });
            }
        }).unwrap();
        *current_quarter_index += 1;
    }
    ///
    pub fn next_population(&mut self, k: usize, mut_const: f64, percentile_gap: usize) {
        let mut new_population: Vec<Player<T>> = Vec::new();
        // 1 player conditional elitism
        // let best = self.find_best();
        // match best {
        //     Some((_, best_player)) => {
        //         if best_player.stocks_sold.len() > 20 {
        //             let mut new_player = best_player.clone();
        //             new_player.soft_reset();
        //             new_population.push(new_player);
        //         } else {
        //             let new_player = self.tourney_select(k).dumb_crossover(self.tourney_select(k), percentile_gap).lazy_mutate(mut_const, percentile_gap);
        //             new_population.push(new_player);
        //         }
        //     }
        //     None => {
        //         let new_player = self.tourney_select(k).dumb_crossover(self.tourney_select(k), percentile_gap).lazy_mutate(mut_const, percentile_gap);
        //         new_population.push(new_player);
        //     }
        // }
        for _i in 0..self.len() {
            let mut new_player = self.tourney_select(k).dumb_crossover(self.tourney_select(k), percentile_gap).lazy_mutate(mut_const, percentile_gap);
            // while self.contains_species(&new_population, &new_player) {
            //     new_player = self.tourney_select(k).dumb_crossover(self.tourney_select(k), percentile_gap).lazy_mutate(mut_const, percentile_gap);
            // }
            new_population.push(new_player);
        }
        self.players = new_population;
    }
    ///
    fn contains_species(&self, new_population: &Vec<Player<T>>, new_player: &Player<T>) -> bool {
        for player in new_population {
            if player.is_same_species_as(new_player) {
                return true;
            }
        }
        false
    }
    /// Perform a tournament selection of size k within the current list of Players. The fitness
    /// function is the current payoff value of each player.
    ///
    /// # Arguments
    /// * `k` - Constant used for tournament selection (default: DEFAULT_TOURNEY_CONST = 3).
    ///
    /// # Remarks
    /// This will fail at runtime if called with k = 0.
    pub fn tourney_select(&self, k: usize) -> &Player<T> {
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
    /// Compute the average percentage gain across the entire population.
    pub fn average_payoff(&self, years: f64) -> f64 {
        let filtered_players = self.iter().filter(|player| player.spend_return > player.spend).collect::<Vec<_>>();
        filtered_players.iter().fold(0.0, |acc, player| acc + player.payoff_per_year(years)) / (filtered_players.len() as f64)
    }
    ///
    pub fn find_best(&self, years: f64) -> Option<(f64, &Player<T>)> {
        let filtered_players = self.iter().filter(|player| player.spend_return > player.spend).collect::<Vec<_>>();
        let mut filtered_players_iter = filtered_players.iter();
        match filtered_players_iter.next() {
            Some(player) => {
                let init_player = *player;
                let init_acc = player.payoff_per_year(years);
                filtered_players_iter.fold(Some((init_acc, init_player)), |acc_tuple, player| {
                    match acc_tuple {
                        Some((acc_payoff, acc_player)) => {
                            let new_payoff = player.payoff_per_year(years);
                            if new_payoff > acc_payoff {
                                Some((new_payoff, player))
                            } else {
                                Some((acc_payoff, acc_player))
                            }
                        }
                        None => None
                    }
                })
            },
            None => {
                None
            }
        }
    }
    ///
    pub fn print_best(&self, years: f64, quarters: &Quarters<T>) {
        match self.find_best(years) {
            Some((payoff, player)) => {
                println!("Best Payoff: {:.3}%, with Screener: {:?}", payoff, player.strategy.format_screen(quarters));
            }
            None => {
                println!("Best Payoff: Didn't exist.");
            }
        }
    }
    /// Calls each players soft reset function.
    pub fn soft_reset(&mut self) {
        for player in &mut self.players {
            player.soft_reset();
        }
    }
    /// Gets the requested index from the players field, as an Option.
    ///
    /// # Arguments
    /// * `index` - The index requested.
    pub fn get(&self, index: usize) -> Option<&Player<T>> {
        self.players.get(index)
    }
    /// Returns the length of the players field.
    pub fn len(&self) -> usize {
        self.players.len()
    }
    /// Returns an iterator over references to the elements in the players variable.
    pub fn iter(&self) -> Iter<Player<T>> {
        self.players.iter()
    }
    /// Returns an iterator over mutable references to the elements in the players variable.
    pub fn iter_mut(&mut self) -> IterMut<Player<T>> {
        self.players.iter_mut()
    }
}
