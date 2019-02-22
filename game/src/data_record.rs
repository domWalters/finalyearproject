use std::{fmt, slice::Iter};

use crate::data_trait::DataTrait;
use crate::player::Player;
use crate::screener::Rule;

#[derive(Debug)]
#[derive(Clone)]
pub struct DataRecord<T: DataTrait> {
    pub record: Vec<T>,
    pub stock_id: StockID
}

#[derive(Debug)]
#[derive(Clone)]
pub struct StockID {
    pub name: String,
    pub time_id: TimeID,
    pub iteration: usize
}

#[derive(Debug)]
#[derive(Clone)]
pub struct TimeID {
    pub year: i64,
    pub quarter: i64
}

impl fmt::Display for TimeID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TimeID[year: {}, quarter: {}]", self.year, self.quarter)
    }
}

impl fmt::Display for StockID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "StockID[name: {:?}, time_id: {}]", self.name, self.time_id)
    }
}

impl<T: DataTrait> fmt::Display for DataRecord<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DataRecord[record: {:?}, stock_id: {}]", self.record, self.stock_id)
    }
}

impl TimeID {
    pub fn is_date(&self, time_id: &TimeID) -> bool {
        (self.quarter == time_id.quarter) & (self.year == time_id.year)
    }

    pub fn is_immediate_previous_of(&self, time_id: &TimeID) -> bool {
        if time_id.quarter != 1 {
            self.quarter + 1 == time_id.quarter
        } else {
            (self.year + 1 == time_id.year) && (self.quarter == 4)
        }
    }

    pub fn after(&self, time_id: &TimeID) -> bool {
        if self.year == time_id.year {
            self.quarter >= time_id.quarter
        } else {
            self.year >= time_id.year
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}-{}", self.year, self.quarter)
    }
}

impl StockID {
    pub fn is_name(&self, stock_id: &StockID) -> bool {
        self.name == stock_id.name
    }

    pub fn is_date(&self, stock_id: &StockID) -> bool {
        self.time_id.is_date(&stock_id.time_id)
    }

    pub fn is_immediate_previous_of(&self, stock_id: &StockID) -> bool {
        self.time_id.is_immediate_previous_of(&stock_id.time_id)
    }

    pub fn to_string(&self) -> String {
        format!("{}-{}", self.name, self.time_id.to_string())
    }
}

impl<T: DataTrait> DataRecord<T> {
    ///
    pub fn is_name<U: DataTrait>(&self, record: &DataRecord<U>) -> bool {
        self.stock_id.is_name(&record.stock_id)
    }
    ///
    pub fn is_date<U: DataTrait>(&self, record: &DataRecord<U>) -> bool {
        self.stock_id.is_date(&record.stock_id)
    }
    /// Returns the length of the DataRecord
    pub fn len(&self) -> usize {
        self.record.len()
    }
    /// Gets a specified indexed element of the DataRecord.
    ///
    /// # Arguments
    /// * `index` - The index requested.
    pub fn get(&self, index: usize) -> T {
        self.record[index]
    }
    ///
    pub fn iter(&self) -> Iter<T> {
        self.record.iter()
    }
    /// Pushes a new element onto the end of the DataRecord.
    ///
    /// # Arguments
    /// * `element` - The element to be pushed.
    pub fn push(&mut self, element: T) {
        self.record.push(element);
    }
    /// Returns true or false based on whether this record has 100*ratio% elements greater than the
    /// corresponding element in the given Player's strategy.
    ///
    /// # Arguments
    /// * `player` - The player who's strategy needs to be checked.
    /// * `ratio` - A number from the interval [0, 1], representing the percentage of elements that
    /// need to be checked successfully.
    pub fn greater_by_ratio(&self, player: &Player<T>, ratio: f64) -> bool {
        let mut true_track = 0;
        let mut false_track = 0;
        let fields_used_count = player.strategy.iter().fold(0, |acc, (_, used, _)| if *used {acc + 1} else {acc});
        let ratio_true_limit = ratio * (fields_used_count as f64);
        let ratio_false_limit = (1.0 - ratio) * (fields_used_count as f64);
        let zip = self.record.iter().zip(player.strategy.iter());
        for (stock_element, (screen_element, field_used, rule)) in zip {
            if *field_used {
                let rule_met = match rule {
                    Rule::Lt => stock_element <= screen_element,
                    Rule::Gt => stock_element >= screen_element
                };
                if rule_met {
                    true_track += 1;
                    if true_track as f64 >= ratio_true_limit {
                        return true;
                    }
                } else {
                    false_track += 1;
                    if false_track as f64 >= ratio_false_limit {
                        return false;
                    }
                }
            }
        }
        true_track as f64 > ratio_true_limit
    }
}
