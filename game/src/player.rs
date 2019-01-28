use std::fmt;

use DataRecord;
use DataSlice;

#[derive(Debug)]
pub struct Player {
    pub strategy: DataSlice,
    pub payoff: f64,
    pub stocks_purchased: Vec<DataRecord>,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Player[strategy: {}, payoff: {}, stocks_purchased: {:?}]", self.strategy, self.payoff, self.stocks_purchased)
    }
}

impl Player {
    /// Creates a blank Player with a blank strategy, a payoff of 0.0, and an empty vector of
    /// stocks_purchased.
    pub fn new_blank() -> Player {
        Player {
            strategy: DataSlice::new_blank(),
            payoff: 0.0,                     // dangerous
            stocks_purchased: Vec::new()
        }
    }
    /// Creates a Player with a uniform random strategy within a set list of boundaries.
    ///
    /// # Arguments
    /// * `l_limits` - The lower limits for each element of the strategy DataSlice.
    /// * `r_limits` - The upper limits for each element of the strategy DataSlice.
    ///
    /// # Remarks
    /// See DataSlice::new_uniform_random() documentation.
    pub fn new_uniform_random((l_limits, r_limits): (&Vec<f64>, &Vec<f64>)) -> Player {
        Player {
            strategy: DataSlice::new_uniform_random((l_limits, r_limits)),
            payoff: 0.0,                     // dangerous
            stocks_purchased: Vec::new()
        }
    }
    /// Resets the player, to have payoff 0.0 and an empty stocks_purchased vector.
    ///
    /// # Remarks
    /// This doesn't create a new Player, it simply edits the old one.
    pub fn reset(&mut self) {
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
            stocks_purchased: Vec::new()
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
            stocks_purchased: Vec::new()
        }
    }
}
