use std::fmt;

use DataSlice;
use Player;

#[derive(Debug)]
pub struct Quarter {
    pub quarter_vector: Vec<DataSlice>,
}

impl fmt::Display for Quarter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Quarter[quarter_vector: {:?}]", self.quarter_vector)
    }
}

impl Quarter {
    // Creates a blank Quarter with a length of zero.
    pub fn load_blank() -> Quarter {
        Quarter {
            quarter_vector: Vec::new()
        }
    }
    /// Assigns to a Player a vector of DataSlices that are piecewise strictly larger than that
    /// Player's set strategy
    ///
    /// # Arguments
    /// * `player` - A Player struct.
    pub fn select_for_player(&self, player: &mut Player) {
        for i in 0..self.quarter_vector.len() {
            if self.quarter_vector[i].greater(&player.strategy) {
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
            match self.find_by_name(stock) {
                Some(current_value) => {
                    player.payoff += current_value.get(index) - stock.get(index)
                },
                None => println!("SERIOUS FUCKING ERROR"),  // stock you bought doesn't exist anymore
            }
        }
    }
    /// Finds a DataSlice (if it exists) that has the same "name" as the input DataSlice.
    ///
    /// # Arguments
    /// * `entry` - A DataSlice to find in the Quarter.
    fn find_by_name<'a>(&self, entry: &'a DataSlice) -> Option<&'a DataSlice> {
        for i in 0..self.quarter_vector.len() {
            if self.quarter_vector[i].name == entry.name {
                return Some(entry)
            }
        }
        return None
    }
}
