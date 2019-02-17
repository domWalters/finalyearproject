use std::{fmt, slice::Iter};
use rand;
use rand::Rng;

#[derive(Debug)]
#[derive(Clone)]
pub enum Rule {
    Lt,
    Gt
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Screener {
    pub screen: Vec<(f64, bool, Rule)>
}

impl fmt::Display for Screener {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Screener[screen: {:?}]", self.screen)
    }
}

impl Screener {
    pub fn soft_reset(&mut self, (l_limits, u_limits): (&Vec<f64>, &Vec<f64>)) {
        let mut rng = rand::thread_rng();
        self.screen = self.screen.iter().zip(l_limits.iter().zip(u_limits)).map(|((_, used, rule), (l, u))| (if l == u {*l} else {rng.gen_range(*l, *u)}, *used, rule.clone())).collect();
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
    pub fn new_uniform_random((l_limits, u_limits): (&Vec<f64>, &Vec<f64>), banned_fields: &Vec<usize>) -> Screener {
        let mut output = Vec::new();
        let mut rng = rand::thread_rng();
        for (i, (l, u)) in l_limits.iter().zip(u_limits).enumerate() {
            let field_used = !banned_fields.contains(&i);
            if l == u {
                output.push((*l, field_used, if rng.gen_bool(0.5) {Rule::Lt} else {Rule::Gt}));
            } else {
                output.push((rng.gen_range(*l, *u), field_used, if rng.gen_bool(0.5) {Rule::Lt} else {Rule::Gt}));
            }
        }
        Screener {
            screen: output
        }
    }
    /// Perform a uniform crossover of two Screeners.
    ///
    /// # Arguments
    /// * `slice` - The Screener to crossover with.
    ///
    /// # Remarks
    /// The resultant Screener is new, and therefore isn't in the memory location of either of
    /// the two that constructed it. This allows the reuse of the Screeners that construct this
    /// crossover.
    pub fn dumb_crossover(&self, slice: &Screener) -> Screener {
        let mut rng = rand::thread_rng();
        Screener {
            screen: self.iter()
                        .zip(slice.iter())
                        .map(|((l, l_used, l_rule), (r, r_used, r_rule))| ((l + r) / 2.0, if rng.gen_bool(0.5) {*l_used} else {*r_used}, if rng.gen_bool(0.5) {l_rule.clone()} else {r_rule.clone()}))
                        .collect()
        }
    }
    /// Perform a lazy mutation on the Screener. This mutation is a per element multiplier
    /// uniformly selected from the interval [0.9, 1.1].
    ///
    /// # Arguments
    /// * `c` - The mutation constant to use for the mutation. On average `c` elements of the
    /// Screener will be mutated.
    ///
    /// # Remarks
    /// This resultant Screener is new, and therefore isn't in the memory location of the Screener
    /// used to create it. This allows the reuse of the Screener that constructs this mutation.
    pub fn lazy_mutate(&self, c: f64) -> Screener {
        let mut rng = rand::thread_rng();
        let percent_mag = 10.0;                         // perform an up to +/-percent_mag% mutation
        Screener {
            screen: self.iter()
                        .map(|(e, used, rule)| {
                            if rng.gen_range(0.0, 1.0) < c / (self.len() as f64) {
                                (*e * rng.gen_range(1.0 - (percent_mag / 100.0), 1.0 + (percent_mag / 100.0)), *used, rule.clone())
                            } else {
                                (*e, *used, rule.clone())
                            }
                        })
                        .collect()
        }
    }
    /// Returns the length of the Screener
    pub fn len(&self) -> usize {
        self.screen.len()
    }

    pub fn iter(&self) -> Iter<(f64, bool, Rule)> {
        self.screen.iter()
    }
}
