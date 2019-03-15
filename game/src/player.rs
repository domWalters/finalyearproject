use std::fmt;

use crate::data_trait::DataTrait;
use crate::data_record::DataRecord;
use crate::screener::Screener;
use crate::screener::Rule;
use crate::quarters::Quarters;

#[derive(Debug)]
#[derive(Clone)]
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
    ///
    pub fn new_player(screener: Screener<T>) -> Player<T> {
        Player {
            strategy: screener,
            spend: 0.0,
            spend_return: 0.0,
            stocks_sold: Vec::new(),
            stocks_purchased: Vec::new(),
        }
    }
    /// Resets the player to have payoff 0, empty stocks vectors, and soft resets the strategies.
    pub fn soft_reset(&mut self) {
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
        if self.spend != 0.0 {self.spend_return / self.spend} else {0.0}
    }
    /// Returns the percent gain of the Player per year.
    ///
    /// # Arguments
    /// * `years` - The number of years that the algorithm has run over.
    pub fn payoff_per_year(&self, years: f64) -> f64 {
        100.0 * (self.payoff().powf(1.0 / years) - 1.0)
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
        let stocks_sold_reward = if self.stocks_sold.len() > 40 {40.0} else {0.0};//self.stocks_sold.len() as f64};
        self.payoff() * (stocks_sold_reward / fields_used_punish)
    }
    ///
    pub fn format_screen<'a>(&'a self, quarters: &'a Quarters<T>) -> Vec<(&String, &Rule, &'a T)> {
        self.strategy.format_screen(quarters)
    }
    ///
    pub fn is_same_species_as(&self, player: &Player<T>) -> bool {
        self.strategy.is_same_species_as(&player.strategy)
    }
}
