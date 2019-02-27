use std::fmt;

use crate::data_trait::DataTrait;
use crate::data_record::DataRecord;
use crate::screener::Screener;
use crate::screener::Rule;
use crate::quarters::Quarters;

#[derive(Debug)]
pub struct Player<T: DataTrait> {
    pub strategy: Screener<T>,
    pub spend: f64,
    pub spend_return: f64,
    pub stocks_sold: Vec<(f64, f64, DataRecord<T>)>,
    pub stocks_purchased: Vec<(f64, DataRecord<T>)>
}

impl<T: DataTrait> fmt::Display for Player<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Player[strategy: {}, spend: {}, spend_return: {}, stocks_sold: {:?}, stocks_purchased: {:?}]", self.strategy, self.spend, self.spend_return, self.stocks_sold, self.stocks_purchased)
    }
}

impl<T: DataTrait> Player<T> {
    /// Creates a Player with a uniform random strategy within a set list of boundaries.
    ///
    /// # Arguments
    /// * `l_limits` - The lower limits for each element of the strategy Screener.
    /// * `r_limits` - The upper limits for each element of the strategy Screener.
    ///
    /// # Remarks
    /// See Screener::new_uniform_random() documentation.
    pub fn new_uniform_random((l_limits, r_limits): (&Vec<T>, &Vec<T>), banned_indicies: &Vec<usize>, percentile_gap: usize) -> Player<T> {
        Player {
            strategy: Screener::new_uniform_random((l_limits, r_limits), banned_indicies, percentile_gap),
            spend: 0.0,
            spend_return: 0.0,
            stocks_sold: Vec::new(),
            stocks_purchased: Vec::new(),
        }
    }
    /// Resets the player to have payoff 0, empty stocks vectors, and soft resets the strategies.
    pub fn soft_reset(&mut self, (l_limits, u_limits): (&Vec<T>, &Vec<T>)) {
        self.strategy.soft_reset((l_limits, u_limits));
        self.spend = 0.0;
        self.spend_return = 0.0;
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
    pub fn dumb_crossover(&self, player: &Player<T>, percentile_gap: usize) -> Player<T> {
        Player {
            strategy: self.strategy.dumb_crossover(&player.strategy, percentile_gap),
            spend: 0.0,
            spend_return: 0.0,
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
    pub fn lazy_mutate(&self, c: f64, percentile_gap: usize) -> Player<T> {
        Player {
            strategy: self.strategy.lazy_mutate(c, percentile_gap),
            spend: 0.0,
            spend_return: 0.0,
            stocks_sold: Vec::new(),
            stocks_purchased: Vec::new()
        }
    }
    /// Returns the percent gain of the Player over the whole timespan.
    pub fn payoff(&self) -> f64 {
        if self.spend != 0.0 {100.0 * ((self.spend_return / self.spend) - 1.0)} else {0.0}
    }
    /// Returns the percent gain of the Player per year.
    ///
    /// # Arguments
    /// * `years` - The number of years that the algorithm has run over.
    pub fn payoff_per_year(&self, years: f64) -> f64 {
        self.payoff().powf(1.0 / years)
    }
    /// Returns the transformed payoff of the Player. The transform punishes long strats and small sold vectors.
    pub fn payoff_transform(&self) -> f64 {
        let field_used_count = self.strategy.iter().fold(0.0, |acc, (_, used, _)| {
            if *used {
                acc + 1.0
            } else {
                acc
            }
        });
        let fields_used_punish = if field_used_count > 10.0 {field_used_count} else if field_used_count < 5.0 {10.0 + 5.0 - field_used_count} else {10.0};
        let stocks_sold_reward = if self.stocks_sold.len() > 20 {20.0} else {self.stocks_sold.len() as f64};
        self.payoff() * (stocks_sold_reward / fields_used_punish)
    }
    ///
    pub fn format_screen<'a>(&'a self, quarters: &'a Quarters<T>) -> Vec<(&String, &Rule, &'a T)> {
        self.strategy.format_screen(quarters)
    }
    /// Recalculate the used variable of the strategy. A field is thrown away if it filters out
    /// less than 0.1% of the training data, or no stock that was successfully bought matched
    /// the rule for that field.
    ///
    /// # Remarks
    /// The 1% stuff in here is now useless (I think).
    pub fn recalc_fields_used(&mut self, compounded_training_vectors: &Vec<Vec<T>>) {
        let mut player_field_counter = vec![0; self.strategy.len()];
        for (_, stock) in &self.stocks_purchased {
            for (k, (strat_element, used, rule)) in self.strategy.iter().enumerate() {
                let rule_met = match rule {
                    Rule::Lt => stock.get(k) <= *strat_element,
                    Rule::Gt => stock.get(k) >= *strat_element
                };
                if rule_met & *used {
                    player_field_counter[k] += 1;
                }
            }
        }
        self.strategy.screen = player_field_counter.iter().zip(self.strategy.iter().zip(compounded_training_vectors.iter())).map(|(&field_count, ((strat_field, used, rule), ref analysis))| {
            if field_count == 0 {
                return (*strat_field, false, rule.clone());
            } else {
                return (*strat_field, *used, rule.clone());
                /*
                let length = analysis.len();
                match rule {
                    Rule::Lt => {
                        for (i, field_analysis) in analysis.iter().rev().enumerate() {
                            if strat_field <= field_analysis {
                                continue;
                            } else {
                                return (*strat_field, (((i + 1) as f64) / (length as f64)) > 0.01, rule.clone());
                            }
                        }
                    },
                    Rule::Gt => {
                        for (i, field_analysis) in analysis.iter().enumerate() {
                            if strat_field >= field_analysis {
                                continue;
                            } else {
                                return (*strat_field, (((i + 1) as f64) / (length as f64)) > 0.01, rule.clone());
                            }
                        }
                    }
                }
                */
            }
            //return (*strat_field, false, rule.clone());   // this line is never hit but needed to compile
        }).collect();
    }
}
