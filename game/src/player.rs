use std::fmt;
use rand;
use rand::Rng;

use crate::data_record::DataRecord;
use crate::screener::Screener;

#[derive(Debug)]
pub struct Player {
    pub strategy: Screener,
    pub payoff: f64,
    pub stocks_purchased: Vec<DataRecord>,
    pub fields_used: Vec<bool>
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Player[strategy: {}, payoff: {}, stocks_purchased: {:?}]", self.strategy, self.payoff, self.stocks_purchased)
    }
}

impl Player {
    /// Creates a Player with a uniform random strategy within a set list of boundaries.
    ///
    /// # Arguments
    /// * `l_limits` - The lower limits for each element of the strategy Screener.
    /// * `r_limits` - The upper limits for each element of the strategy Screener.
    ///
    /// # Remarks
    /// See Screener::new_uniform_random() documentation.
    pub fn new_uniform_random((l_limits, r_limits): (&Vec<f64>, &Vec<f64>)) -> Player {
        Player {
            strategy: Screener::new_uniform_random((l_limits, r_limits)),
            payoff: 0.0,                     // dangerous
            stocks_purchased: Vec::new(),
            fields_used: vec![true; l_limits.len()]
        }
    }
    /// Resets the player to have payoff 0, an empty stocks_purchased vector, and every field being used.
    pub fn reset(&mut self) {
        self.payoff = 0.0;
        self.stocks_purchased = Vec::new();
        self.fields_used = vec![true; self.strategy.len()];
    }
    /// Resets the player to have payoff 0, and an empty stocks_purchased vector.
    pub fn soft_reset(&mut self) {
        self.payoff = 0.0;
        self.stocks_purchased = Vec::new();
    }
    /// Perform a uniform crossover of two Players.
    ///
    /// # Arguments
    /// * `player` - The Player object to be crossed with.
    ///
    /// # Remarks
    /// The resultant Player is new, and therefore isn't in the memory location of either of
    /// the two that constructed it. This allows the reuse of the Players that construct this
    /// crossover. The payoff and stocks_purchased entries are reset. The fields_used entry has
    /// it's elements picked randomly from either player.
    pub fn dumb_crossover(&self, player: &Player) -> Player {
        Player {
            strategy: self.strategy.dumb_crossover(&player.strategy),
            payoff: 0.0,
            stocks_purchased: Vec::new(),
            fields_used: self.fields_used.iter()
                                         .zip(player.fields_used.iter())
                                         .map(|(l, r)| {
                                             let mut rng = rand::thread_rng();
                                             if rng.gen_bool(0.5) {
                                                 *l
                                             } else {
                                                 *r
                                             }
                                         }).collect()
        }
    }
    /// Perform a lazy mutation on the Player.
    ///
    /// # Arguments
    /// * `c` - The mutation constant to use for the mutation.
    ///
    /// # Remarks
    /// This resultant Player is new, and therefore isn't in the memory location of the Player
    /// used to create it. This allows the reuse of the Player that constructs this mutation.
    /// The payoff and stocks_purchased entries are reset.
    pub fn lazy_mutate(&self, c: f64) -> Player {
        Player {
            strategy: self.strategy.lazy_mutate(c),
            payoff: 0.0,
            stocks_purchased: Vec::new(),
            fields_used: self.fields_used.clone()
        }
    }
    ///
    pub fn payoff_normalise(&mut self) {
        let sym_length = 1.0 + (self.stocks_purchased.len() as f64 / 400.0);
        self.payoff = self.payoff * (sym_length / (self.fields_used.iter().fold(0.0, |acc, &used| {
            if used {
                acc + 1.0
            } else {
                acc
            }
        }) * 0.25));
    }
    ///
    pub fn recalc_fields_used(&mut self, compounded_training_vectors: &Vec<Vec<f64>>) {
        let mut player_field_counter = vec![0; self.strategy.len()];
        for stock in &self.stocks_purchased {
            for k in 0..self.strategy.len() {
                if (stock.get(k) > self.strategy.get(k)) & *self.fields_used.get(k).unwrap() {
                    player_field_counter[k] += 1;
                }
            }
        }
        self.fields_used = player_field_counter.iter().zip(self.strategy.screen.iter().zip(compounded_training_vectors.iter())).map(|(&field_count, (strat_field, ref analysis))| {
            if field_count == 0 {
                return false;
            } else {
                let length = analysis.len();
                for (i, field_analysis) in analysis.iter().enumerate() {
                    if strat_field > field_analysis {
                        continue;
                    } else {
                        return (((i + 1) as f64) / (length as f64)) > 0.01;
                    }
                }
                return false;   // this line is never hit but needed to compile
            }
        }).collect();
    }
}
