use std::fmt;

use Screener;

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

impl IntoIterator for DataRecord {
    type Item = f64;
    type IntoIter = ::std::vec::IntoIter<f64>;

    fn into_iter(self) -> Self::IntoIter {
        self.record.into_iter()
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
    ///
    pub fn push(&mut self, new_element: f64) {
        self.record.push(new_element);
    }

    ///
    pub fn greater_by_ratio(&self, screen: &Screener, ratio: f64) -> bool {
        let mut true_track = 0;
        let mut false_track = 0;
        for i in 0..self.len() {
            if self.get(i) >= screen.get(i) {
                true_track += 1;
            } else {
                false_track += 1;
            }
            if (true_track as f64) / (self.len() as f64) > ratio {
                return true;
            } else if (false_track as f64) / (self.len() as f64) > 1.0 - ratio {
                return false;
            }
        }
        (true_track as f64) / (self.len() as f64) > ratio
    }
}
