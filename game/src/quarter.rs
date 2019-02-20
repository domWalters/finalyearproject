use std::{fmt, slice::Iter};

use crate::data_record::{DataRecord, StockID, TimeID};
use crate::player::Player;

#[derive(Debug)]
#[derive(Clone)]
pub struct Quarter {
    pub quarter_vector: Vec<DataRecord>,
    pub time_id: TimeID
}

impl fmt::Display for Quarter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Quarter[quarter_vector: {:?}, time_id: {}]", self.quarter_vector, self.time_id)
    }
}

impl Quarter {
    /// Creates a blank Quarter with a length of zero.
    ///
    /// # Arguments
    /// * `year` - The year that this Quarter is from.
    /// * `quarter` - The quarter that this Quarter represents.
    pub fn load_blank(year: i64, quarter: i64) -> Quarter {
        Quarter {
            quarter_vector: Vec::new(),
            time_id: TimeID {
                year: year,
                quarter: quarter
            }
        }
    }
    /// Returns the length of the quarter_vector field.
    pub fn len(&self) -> usize {
        self.quarter_vector.len()
    }
    /// Gets the requested index from the quarter_vector field, as an Option.
    ///
    /// # Arguments
    /// * `index` - The index requested.
    pub fn get(&self, index: usize) -> Option<&DataRecord> {
        self.quarter_vector.get(index)
    }
    /// Pushes a new DataRecord onto the end of the Quarter.
    ///
    /// # Arguments
    /// * `new_record` - The record to be pushed.
    pub fn push(&mut self, new_record: DataRecord) {
        self.quarter_vector.push(new_record);
    }
    /// Returns an iterator over references to the elements in the quarter_vector variable of the
    /// Quarter.
    pub fn iter(&self) -> Iter<DataRecord> {
        self.quarter_vector.iter()
    }
    /// Removes the DataRecord in the index provided, and returns it.
    ///
    /// # Arguments
    /// * `index` - The index of the element to be removed and returned.
    pub fn remove(&mut self, index: usize) -> DataRecord {
        self.quarter_vector.remove(index)
    }
    /// Assigns to a Player a vector of DataRecords that are piecewise strictly larger than that
    /// Player's set strategy.
    ///
    /// # Arguments
    /// * `player` - A Player struct.
    ///
    /// # Remarks
    /// This function is overly convoluted.
    pub fn select_for_player(&self, player: &mut Player, ratio: f64, index: usize, iteration: usize) {
        // Buy from quarter
        for stock in &self.quarter_vector {
            if stock.greater_by_ratio(&player, ratio) & (stock.stock_id.iteration == iteration) {
                player.stocks_purchased.push(stock.clone());
            }
        }
        // Sell discontinuous stocks, create a list of what to sell
        let mut indicies_to_bin: Vec<(usize, StockID)> = Vec::new();
        for (i, stock) in player.stocks_purchased.iter().enumerate() {  // THIS ITER IS ORDERED BY DEFINITION
            if stock.stock_id.time_id.is_date(&self.time_id) {
                let mut indicies_to_save: Vec<usize> = Vec::new();
                for (j, (_, bin_stock_id)) in indicies_to_bin.iter().enumerate() {
                    if stock.stock_id.is_name(&bin_stock_id) {
                        if bin_stock_id.is_immediate_previous_of(&stock.stock_id) {
                            indicies_to_save.push(j);
                        } else {
                            continue;
                        }
                    } else {
                        continue;
                    }
                }
                // Now remove those in the save list from the bin list
                for j in indicies_to_save.iter().rev() {
                    indicies_to_bin.remove(*j);
                }
            } else {
                indicies_to_bin.push((i, stock.stock_id.clone()));  // throw away everything not in this quarter
            }
        }
        // Fully constructed bin list, construct payoff and chuck
        for i in indicies_to_bin.iter().rev().map(|(i, _stock)| i) {
            match self.find_by_stock_name(&player.stocks_purchased[*i]) {
                Some(current_value) => {
                    player.payoff += 100.0 * ((current_value.get(index) / player.stocks_purchased[*i].get(index)) - 1.0);
                },
                None => {
                    player.payoff += 0.0;
                }
            }
            player.stocks_sold.push(player.stocks_purchased.remove(*i));
        }
    }
    /// Calculates a payoff given to a player based on the value of the stocks that were purchased.
    ///
    /// # Arguments
    /// * `player` - A Player struct that provides a list of purchased stocks and is used to store
    /// the payoff value that is calculated.
    /// * `index` - The index in the stock DataRecord to use for the payoff calculation.
    ///
    /// # Remarks
    /// This payoff is relative, so as not to benefit stocks with large values more than lower value stocks.
    pub fn calc_payoffs(&self, player: &mut Player, index: usize) {
        for stock in &player.stocks_purchased {
            match self.find_by_stock_name(&stock) {
                Some(current_value) => {
                    player.payoff += 100.0 * ((current_value.get(index) / stock.get(index)) - 1.0);
                },
                None => {
                    player.payoff += 0.0;
                }
            }
            player.stocks_sold.push(stock.clone());
        }
    }
    /// Finds a DataRecord (if it exists) that has the same ".stock_id.name" as the input DataRecord.
    ///
    /// # Arguments
    /// * `entry` - A DataRecord to find in the Quarter.
    pub fn find_by_stock_name<'a>(&'a self, entry: &DataRecord) -> Option<&'a DataRecord> {
        for stock in &self.quarter_vector {
            if stock.is_name(entry) {
                return Some(&stock)
            }
        }
        println!("ERROR: Stock no longer exists - {:?}", entry.stock_id.name);
        return None
    }
}
