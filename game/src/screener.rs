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

impl Screener {
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
            screen: self.screen
                        .iter()
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
        let percent_mag = 10.0;                     // up to +/- 10% mutation
        Screener {
            screen: self.screen
                        .iter()
                        .map(|&e| {
                            if rng.gen_range(0.0, 1.0) < c / (self.len() as f64) {
                                e * rng.gen_range(1.0 - (percent_mag / 100.0), 1.0 + (percent_mag / 100.0)) // perform an up to +/-percent_mag% mutation
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
}
