use std::fmt;

use crate::data_record::DataRecord;
use crate::screener::Screener;

#[derive(Debug)]
pub struct Player {
    pub strategy: Screener,
    pub payoff: f64,
    pub stocks_sold: Vec<DataRecord>,
    pub stocks_purchased: Vec<DataRecord>
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Player[strategy: {}, payoff: {}, stocks_sold: {:?}, stocks_purchased: {:?}]", self.strategy, self.payoff, self.stocks_sold, self.stocks_purchased)
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
    pub fn new_uniform_random((l_limits, r_limits): (&Vec<f64>, &Vec<f64>), banned_indicies: &Vec<usize>) -> Player {
        Player {
            strategy: Screener::new_uniform_random((l_limits, r_limits), banned_indicies),
            payoff: 0.0,                     // dangerous
            stocks_sold: Vec::new(),
            stocks_purchased: Vec::new(),
        }
    }
    /// Resets the player to have payoff 0, and an empty stocks_purchased vector.
    pub fn soft_reset(&mut self) {
        self.payoff = 0.0;
        self.stocks_sold = Vec::new();
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
            stocks_sold: Vec::new(),
            stocks_purchased: Vec::new()
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
            stocks_sold: Vec::new(),
            stocks_purchased: Vec::new()
        }
    }
    ///
    pub fn payoff_normalise(&mut self) {    // change the punishment for long field lists to be constant below a certain length
        if self.stocks_sold.len() != 0 {
            let field_used_symbolic_length = self.strategy.iter().fold(0.0, |acc, (_, used)| {
                if *used {
                    acc + 1.0
                } else {
                    acc
                }
            });
            self.payoff = (self.payoff * (4.0 / if field_used_symbolic_length > 5.0 {field_used_symbolic_length} else {5.0})) * (self.stocks_sold.len() as f64);
        } else {
            self.payoff = 0.0;
        }
    }
    ///
    pub fn recalc_fields_used(&mut self, compounded_training_vectors: &Vec<Vec<f64>>) {
        let mut player_field_counter = vec![0; self.strategy.len()];
        for stock in &self.stocks_purchased {
            for (k, (strat_element, used)) in self.strategy.iter().enumerate() {
                if (stock.get(k) > *strat_element) & *used {
                    player_field_counter[k] += 1;
                }
            }
        }
        self.strategy.screen = player_field_counter.iter().zip(self.strategy.iter().zip(compounded_training_vectors.iter())).map(|(&field_count, ((strat_field, _), ref analysis))| {
            if field_count == 0 {
                return (*strat_field, false);
            } else {
                let length = analysis.len();
                for (i, field_analysis) in analysis.iter().enumerate() {
                    if strat_field > field_analysis {
                        continue;
                    } else {
                        return (*strat_field, (((i + 1) as f64) / (length as f64)) > 0.01);
                    }
                }
                return (*strat_field, false);   // this line is never hit but needed to compile
            }
        }).collect();
    }
}
