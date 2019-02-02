use std::fmt;

use Player;

#[derive(Debug)]
#[derive(Clone)]
pub struct DataRecord {
    pub record: Vec<f64>,
    pub stock_id: StockID
}

#[derive(Debug)]
#[derive(Clone)]
pub struct StockID {
    pub name: String,
    pub year: i64,
    pub quarter: i64
}

impl fmt::Display for StockID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "StockID[name: {:?}, year: {}, quarter: {}]", self.name, self.year, self.quarter)
    }
}

impl fmt::Display for DataRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DataRecord[record: {:?}, stock_id: {}]", self.record, self.stock_id)
    }
}

impl DataRecord {
    /// Returns the length of the DataRecord
    pub fn len(&self) -> usize {
        self.record.len()
    }
    /// Gets a specified indexed element of the DataRecord.
    ///
    /// # Arguments
    /// * `index` - The index requested.
    pub fn get(&self, index: usize) -> f64 {
        self.record[index]
    }
    /// Pushes a new element onto the end of the DataRecord.
    ///
    /// # Arguments
    /// * `element` - The element to be pushed.
    pub fn push(&mut self, element: f64) {
        self.record.push(element);
    }
    /// Returns true or false based on whether this record has 100*ratio% elements greater than the
    /// corresponding element in the given Player's strategy.
    ///
    /// # Arguments
    /// * `player` - The player who's strategy needs to be checked.
    /// * `ratio` - A number from the interval [0, 1], representing the percentage of elements that
    /// need to be checked successfully.
    pub fn greater_by_ratio(&self, player: &Player, ratio: f64) -> bool {
        let mut true_track = 0;
        let mut false_track = 0;
        let fields_used_count = player.fields_used.iter()
                                                  .fold(0, |acc, &next| {
                                                      if next {
                                                          acc + 1
                                                      } else {
                                                          acc
                                                      }
                                                  });
        let ratio_true_limit = ratio * (fields_used_count as f64);
        let ratio_false_limit = (1.0 - ratio) * (fields_used_count as f64);
        let zip = self.record.iter().zip(player.strategy.screen.iter()).zip(player.fields_used.iter());
        for ((&stock_element, &screen_element), &field_used) in zip {
            if field_used {
                if stock_element >= screen_element {
                    true_track += 1;
                    if true_track as f64 > ratio_true_limit {
                        return true;
                    }
                } else {
                    false_track += 1;
                    if false_track as f64 > ratio_false_limit {
                        return false;
                    }
                }
            }
        }
        true_track as f64 > ratio_true_limit
    }
}
