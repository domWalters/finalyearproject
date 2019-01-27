use std::fmt;
use rand::Rng;

#[derive(Debug)]
#[derive(Clone)]
pub struct DataSlice {
    pub slice_vector: Vec<f64>,
    pub name: String,
}

impl fmt::Display for DataSlice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DataSlice[slice_vector: {:?}, name: {}]", self.slice_vector, self.name)
    }
}

impl IntoIterator for DataSlice {
    type Item = f64;
    type IntoIter = ::std::vec::IntoIter<f64>;

    fn into_iter(self) -> Self::IntoIter {
        self.slice_vector.into_iter()
    }
}

impl DataSlice {
    /// Creates a blank DataSlice with the name "" and a length of zero.
    pub fn new_blank() -> DataSlice {
        DataSlice {
            slice_vector: Vec::new(),
            name: "".to_string(),
        }
    }
    /// Creates a uniform random DataSlice within a set list of boundaries.
    ///
    /// # Arguments
    /// * `l_limits` - The lower limits for each element of the DataSlice.
    /// * `r_limits` - The upper limits for each element of the DataSlice.
    ///
    /// # Remarks
    /// Each argument is a vector that is as long as the DataSlice that needs to be generated.
    /// The ith element of the DataSlice is greater than the ith element of l_limits, and less than
    /// the ith element of r_limits.
    pub fn new_uniform_random((l_limits, r_limits): (&Vec<f64>, &Vec<f64>)) -> DataSlice {
        let mut output = Vec::new();
        let mut rng = rand::thread_rng();
        for (l, r) in l_limits.iter().zip(r_limits) {
            if l == r {
                output.push(*l);
            } else {
                output.push(rng.gen_range(*l, *r));
            }
        }
        DataSlice {
            slice_vector: output,
            name: "".to_string(),
        }
    }
    /// Perform a uniform crossover of two DataSlices.
    ///
    /// # Arguments
    /// * `slice` - The DataSlice to crossover with.
    ///
    /// # Remarks
    /// The resultant DataSlice is new, and therefore isn't in the memory location of either of
    /// the two that constructed it. This allows the reuse of the DataSlices that construct this
    /// crossover.
    pub fn dumb_crossover(&self, slice: &DataSlice) -> DataSlice {
        DataSlice {
            slice_vector: self.clone()
                              .into_iter()
                              .zip(slice.slice_vector.iter())
                              .map(|(l, r)| (l + r) / 2.0)
                              .collect(),
            name: "".to_string()
        }
    }
    /// Perform a mutation on the DataSlice.
    ///
    /// # Arguments
    /// * `c` - The mutation constant to use for the mutation.
    ///
    /// # Remarks
    /// This resultant DataSlice is new, and therefore isn't in the memory location of the DataSlice
    /// used to create it. This allows the reuse of the DataSlice that constructs this mutation.
    pub fn mutate(&self, c: f64) -> DataSlice {
        for _i in 0..0 {
            // For now, don't do anything.
            // This will need to normal randomise for each variable.
            // Each element will need a different amount of randomness.
        }
        self.clone()
    }

    pub fn lazy_mutate(&self, c: f64) -> DataSlice {    // does the mutate roll per element not per vector
        let mut rng = rand::thread_rng();
        DataSlice {
            slice_vector: self.clone().into_iter()
                              .map(|e| {
                                  if rng.gen_range(0.0, 1.0) < c / (self.len() as f64) {
                                      e * 1.1 * rng.gen_range(10.0 / 11.0, 1.0)   // perform an up to 10% mutate
                                  } else {
                                      e
                                  }
                              })
                              .collect(),
            name: "".to_string()
        }
    }
    /// Returns the length of the DataSlice
    pub fn len(&self) -> usize {
        self.slice_vector.len()
    }
    /// Gets a specified indexed element of the DataSlice.
    ///
    /// # Arguments
    /// * `index` - The index requested.
    pub fn get(&self, index: usize) -> f64 {
        self.slice_vector[index]
    }
    /// Returns a boolean representing whether the argument DataSlice is piecewise strictly
    /// larger than the current DataSlice.
    ///
    /// # Arguments
    /// * `slice` - The DataSlice to compare against.
    pub fn greater(&self, slice: &DataSlice) -> bool {
        for i in 0..self.len() {
            if slice.get(i) > self.get(i) {
                return false
            }
        }
        true
    }

    pub fn greater_by_ratio(&self, slice: &DataSlice, ratio: f64) -> bool {
        let mut true_track = 0;
        let mut false_track = 0;
        for i in 0..self.len() {
            if self.get(i) >= slice.get(i) {
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

    pub fn stock_name(&self) -> &str {
        self.name.split('-').next().unwrap()
    }
}
