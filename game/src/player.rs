use std::fmt;
use rand::Rng;

use DataRecord;
use Screener;

#[derive(Debug)]
pub struct Player {
    pub strategy: Screener,
    pub payoff: f64,
    pub stocks_purchased: Vec<DataRecord>,
    pub fields_used: Vec<bool>
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Player[strategy: {}, payoff: {}, stocks_purchased: {:?}]", self.strategy, self.payoff, self.stocks_purchased)
    }
}

impl Player {
    /// Creates a Player with a uniform random strategy within a set list of boundaries.
    ///
    /// # Arguments
    /// * `l_limits` - The lower limits for each element of the strategy Screener.
    /// * `r_limits` - The upper limits for each element of the strategy Screener.
    ///
    /// # Remarks
    /// See Screener::new_uniform_random() documentation.
    pub fn new_uniform_random((l_limits, r_limits): (&Vec<f64>, &Vec<f64>)) -> Player {
        Player {
            strategy: Screener::new_uniform_random((l_limits, r_limits)),
            payoff: 0.0,                     // dangerous
            stocks_purchased: Vec::new(),
            fields_used: vec![true; l_limits.len()]
        }
    }
    /// Resets the player, to have payoff 0.0 and an empty stocks_purchased vector.
    ///
    /// # Remarks
    /// This doesn't create a new Player, it simply edits the old one.
    pub fn reset(&mut self) {
        self.payoff = 0.0;
        self.stocks_purchased = Vec::new();
        self.fields_used = vec![true; self.strategy.len()];
    }

    pub fn soft_reset(&mut self) {
        self.payoff = 0.0;
        self.stocks_purchased = Vec::new();
    }
    /// Perform a uniform crossover of two Players.
    ///
    /// # Arguments
    /// * `player` - The Player object to be crossed with.
    ///
    /// # Remarks
    /// The resultant Player is new, and therefore isn't in the memory location of either of
    /// the two that constructed it. This allows the reuse of the Players that construct this
    /// crossover. The payoff and stocks_purchased entries are reset.
    pub fn dumb_crossover(&self, player: &Player) -> Player {
        Player {
            strategy: self.strategy.dumb_crossover(&player.strategy),
            payoff: 0.0,
            stocks_purchased: Vec::new(),
            fields_used: self.fields_used.iter()
                                         .zip(player.fields_used.iter())
                                         .map(|(l, r)| {
                                             let mut rng = rand::thread_rng();
                                             if rng.gen_bool(0.5) {
                                                 *l
                                             } else {
                                                 *r
                                             }
                                         }).collect()
        }
    }
    /// Perform a lazy mutation on the Player.
    ///
    /// # Arguments
    /// * `c` - The mutation constant to use for the mutation.
    ///
    /// # Remarks
    /// This resultant Player is new, and therefore isn't in the memory location of the Player
    /// used to create it. This allows the reuse of the Player that constructs this mutation.
    /// The payoff and stocks_purchased entries are reset.
    pub fn lazy_mutate(&self, c: f64) -> Player {
        Player {
            strategy: self.strategy.lazy_mutate(c),
            payoff: 0.0,
            stocks_purchased: Vec::new(),
            fields_used: self.fields_used.clone()
        }
    }
}
