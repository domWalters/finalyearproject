use std::fmt;
use rand::Rng;

#[derive(Debug)]
#[derive(Clone)]
pub struct Screener {
    pub screen: Vec<f64>
}

impl fmt::Display for Screener {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Screener[screen: {:?}]", self.screen)
    }
}

impl IntoIterator for Screener {
    type Item = f64;
    type IntoIter = ::std::vec::IntoIter<f64>;

    fn into_iter(self) -> Self::IntoIter {
        self.screen.into_iter()
    }
}

impl Screener {
    /// Creates a blank Screener with the name "" and a length of zero.
    pub fn new_blank() -> Screener {
        Screener {
            screen: Vec::new()
        }
    }
    /// Creates a uniform random Screener within a set list of boundaries.
    ///
    /// # Arguments
    /// * `l_limits` - The lower limits for each element of the Screener.
    /// * `r_limits` - The upper limits for each element of the Screener.
    ///
    /// # Remarks
    /// Each argument is a vector that is as long as the Screener that needs to be generated.
    /// The ith element of the Screener is greater than the ith element of l_limits, and less than
    /// the ith element of r_limits.
    pub fn new_uniform_random((l_limits, r_limits): (&Vec<f64>, &Vec<f64>)) -> Screener {
        let mut output = Vec::new();
        let mut rng = rand::thread_rng();
        for (l, r) in l_limits.iter().zip(r_limits) {
            if l == r {
                output.push(*l);
            } else {
                output.push(rng.gen_range(*l, *r));
            }
        }
        Screener {
            screen: output
        }
    }
    /// Perform a uniform crossover of two DataSlices.
    ///
    /// # Arguments
    /// * `slice` - The Screener to crossover with.
    ///
    /// # Remarks
    /// The resultant Screener is new, and therefore isn't in the memory location of either of
    /// the two that constructed it. This allows the reuse of the DataSlices that construct this
    /// crossover.
    pub fn dumb_crossover(&self, slice: &Screener) -> Screener {
        Screener {
            screen: self.clone()
                              .into_iter()
                              .zip(slice.screen.iter())
                              .map(|(l, r)| (l + r) / 2.0)
                              .collect()
        }
    }
    /// Perform a lazy mutation on the Screener.
    ///
    /// # Arguments
    /// * `c` - The mutation constant to use for the mutation.
    ///
    /// # Remarks
    /// This resultant Screener is new, and therefore isn't in the memory location of the Screener
    /// used to create it. This allows the reuse of the Screener that constructs this mutation.
    pub fn lazy_mutate(&self, c: f64) -> Screener {    // does the mutate roll per element not per vector
        let mut rng = rand::thread_rng();
        Screener {
            screen: self.clone().into_iter()
                              .map(|e| {
                                  if rng.gen_range(0.0, 1.0) < c / (self.len() as f64) {
                                      e * 1.1 * rng.gen_range(10.0 / 11.0, 1.0)   // perform an up to 10% mutate
                                  } else {
                                      e
                                  }
                              })
                              .collect()
        }
    }
    /// Returns the length of the Screener
    pub fn len(&self) -> usize {
        self.screen.len()
    }
    /// Gets a specified indexed element of the Screener.
    ///
    /// # Arguments
    /// * `index` - The index requested.
    pub fn get(&self, index: usize) -> f64 {
        self.screen[index]
    }
    /// Returns a boolean representing whether the argument Screener is piecewise strictly
    /// larger than the current Screener.
    ///
    /// # Arguments
    /// * `slice` - The Screener to compare against.
    pub fn greater(&self, slice: &Screener) -> bool {
        for i in 0..self.len() {
            if slice.get(i) > self.get(i) {
                return false
            }
        }
        true
    }

    pub fn greater_by_ratio(&self, slice: &Screener, ratio: f64) -> bool {
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
}
