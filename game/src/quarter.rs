use std::{fmt, slice::Iter};

use crate::data_trait::DataTrait;
use crate::data_record::{DataRecord, StockID, TimeID};
use crate::player::Player;

#[derive(Debug)]
#[derive(Clone)]
pub struct Quarter<T: DataTrait> {
    pub quarter_vector: Vec<DataRecord<T>>,
    pub time_id: TimeID
}

impl<T: DataTrait> fmt::Display for Quarter<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Quarter[quarter_vector: {:?}, time_id: {}]", self.quarter_vector, self.time_id)
    }
}

impl<T: DataTrait> Quarter<T> {
    /// Creates a blank Quarter with a length of zero.
    ///
    /// # Arguments
    /// * `year` - The year that this Quarter is from.
    /// * `quarter` - The quarter that this Quarter represents.
    pub fn load_blank(year: i64, quarter: i64) -> Quarter<T> {
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
    pub fn get(&self, index: usize) -> Option<&DataRecord<T>> {
        self.quarter_vector.get(index)
    }
    /// Pushes a new DataRecord onto the end of the Quarter.
    ///
    /// # Arguments
    /// * `new_record` - The record to be pushed.
    pub fn push(&mut self, new_record: DataRecord<T>) {
        self.quarter_vector.push(new_record);
    }
    /// Returns an iterator over references to the elements in the quarter_vector variable of the
    /// Quarter.
    pub fn iter(&self) -> Iter<DataRecord<T>> {
        self.quarter_vector.iter()
    }
    /// Removes the DataRecord in the index provided, and returns it.
    ///
    /// # Arguments
    /// * `index` - The index of the element to be removed and returned.
    pub fn remove(&mut self, index: usize) -> DataRecord<T> {
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
    pub fn select_for_player(&self, float_quarter: &Quarter<f64>, player: &mut Player<T>, index: usize, iteration: usize) {
        // Buy from quarter
        for (stock, stock_float) in self.iter().zip(float_quarter.iter()) {
            if stock.is_satisfied_by(&player) & (stock.stock_id.iteration == iteration) {
                player.stocks_purchased.push((stock_float.get(index).unwrap(), stock.clone()));
            }
        }
        // Sell discontinuous stocks, create a list of what to sell
        let mut indicies_to_bin: Vec<(usize, StockID)> = Vec::new();
        for (i, (_, stock)) in player.stocks_purchased.iter().enumerate() {  // THIS ITER IS ORDERED BY DEFINITION
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
            let (buy_price, stock) = &player.stocks_purchased[*i];
            let sell_price;
            match float_quarter.find_by_stock_name(stock) {
                Some(current_value) => {
                    sell_price = current_value.get(index).unwrap();
                    player.spend += buy_price;
                    player.spend_return += sell_price;
                },
                None => {   // stock no longer exists, you bought and couldn't sell
                    sell_price = *buy_price;
                    player.spend += buy_price;
                }
            }
            let (buy_price, stock_removed) = player.stocks_purchased.remove(*i);
            player.stocks_sold.push((buy_price, sell_price, stock_removed));
        }
    }
    /// Finds a DataRecord (if it exists) that has the same ".stock_id.name" as the input DataRecord.
    ///
    /// # Arguments
    /// * `entry` - A DataRecord to find in the Quarter.
    pub fn find_by_stock_name<'a, U: DataTrait>(&'a self, entry: &DataRecord<U>) -> Option<&'a DataRecord<T>> {
        for stock in &self.quarter_vector {
            if stock.is_name(entry) {
                return Some(&stock)
            }
        }
        println!("ERROR: Stock no longer exists - {:?}", entry.stock_id.name);
        return None
    }
}
