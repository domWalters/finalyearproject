use std::fmt;

use DataRecord;
use Player;

#[derive(Debug)]
pub struct Quarter {
    pub quarter_vector: Vec<DataRecord>,
    pub year: i64,
    pub quarter: i64
}

impl fmt::Display for Quarter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Quarter[quarter_vector: {:?}, year: {:?}, quarter: {:?}]", self.quarter_vector, self.year, self.quarter)
    }
}

impl IntoIterator for Quarter {
    type Item = DataRecord;
    type IntoIter = ::std::vec::IntoIter<DataRecord>;

    fn into_iter(self) -> Self::IntoIter {
        self.quarter_vector.into_iter()
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

    pub fn get(&self, index: usize) -> Option<&DataRecord> {
        self.quarter_vector.get(index)
    }

    pub fn push(&mut self, new_record: DataRecord) {
        self.quarter_vector.push(new_record);
    }
    /// Assigns to a Player a vector of DataRecords that are piecewise strictly larger than that
    /// Player's set strategy
    ///
    /// # Arguments
    /// * `player` - A Player struct.
    pub fn select_for_player(&self, player: &mut Player) {
        for stock in &self.quarter_vector {
            if stock.greater_by_ratio(&player.strategy, 0.5) {
                player.stocks_purchased.push(stock.clone());
            }
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
                    player.payoff += current_value.get(index) / (stock.get(index) * (player.stocks_purchased.len() as f64));
                },
                None => return,
            }
        }
    }
    /// Finds a DataRecord (if it exists) that has the same ".stock_id.name" as the input DataRecord.
    ///
    /// # Arguments
    /// * `entry` - A DataRecord to find in the Quarter.
    fn find_by_stock_name<'a>(&'a self, entry: &DataRecord) -> Option<&'a DataRecord> {
        for stock in &self.quarter_vector {
            if stock.stock_id.name == entry.stock_id.name {
                return Some(&stock)
            }
        }
        println!("ERROR: Stock no longer exists - {:?}", entry.stock_id.name);
        return None
    }
}
