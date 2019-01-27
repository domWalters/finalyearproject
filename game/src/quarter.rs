use std::fmt;

use DataSlice;
use Player;

#[derive(Debug)]
pub struct Quarter {
    pub quarter_vector: Vec<DataSlice>,
    pub year: i64,
    pub quarter: i64
}

impl fmt::Display for Quarter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Quarter[quarter_vector: {:?}, year: {:?}, quarter: {:?}]", self.quarter_vector, self.year, self.quarter)
    }
}

impl Quarter {
    // Creates a blank Quarter with a length of zero.
    pub fn load_blank(year: i64, quarter: i64) -> Quarter {
        Quarter {
            quarter_vector: Vec::new(),
            year: year,
            quarter: quarter
        }
    }

    pub fn len(&self) -> usize {
        self.quarter_vector.len()
    }

    pub fn get(&self, index: usize) -> Option<&DataSlice> {
        self.quarter_vector.get(index)
    }
    /// Assigns to a Player a vector of DataSlices that are piecewise strictly larger than that
    /// Player's set strategy
    ///
    /// # Arguments
    /// * `player` - A Player struct.
    pub fn select_for_player(&self, player: &mut Player) {
        for i in 0..self.quarter_vector.len() {
            if self.quarter_vector[i].greater_by_ratio(&player.strategy, 0.5) {
                player.stocks_purchased.push(self.quarter_vector[i].copy());
            }
        }
    }
    /// Calculates a payoff given to a player based on the value of the stocks that were purchased.
    ///
    /// # Arguments
    /// * `player` - A Player struct that provides a list of purchased stocks and is used to store
    /// the payoff value that is calculated.
    /// * `index` - The index in the stock DataSlice to use for the payoff calculation.
    pub fn calc_payoffs(&self, player: &mut Player, index: usize) {
        for j in 0..player.stocks_purchased.len() {
            let stock = &player.stocks_purchased[j];
            match self.find_by_stock_name(stock) {
                Some(current_value) => {
                    player.payoff += current_value.get(index) - stock.get(index);
                },
                None => return,
            }
        }
    }
    /// Finds a DataSlice (if it exists) that has the same "stock_name()" as the input DataSlice.
    ///
    /// # Arguments
    /// * `entry` - A DataSlice to find in the Quarter.
    fn find_by_stock_name<'a>(&'a self, entry: &DataSlice) -> Option<&'a DataSlice> {
        let entry_name = entry.stock_name();
        for i in 0..self.quarter_vector.len() {
            if self.quarter_vector[i].stock_name() == entry_name {
                return Some(&self.quarter_vector[i])
            }
        }
        println!("ERROR: Stock no longer exists - {:?}", entry_name);
        return None
    }
}
