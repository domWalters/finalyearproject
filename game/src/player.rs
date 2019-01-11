use std::fmt;

use DataSlice;

#[derive(Debug)]
pub struct Player {
    pub strategy: DataSlice,
    pub payoff: f64,
    pub stocks_purchased: Vec<DataSlice>,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Player[strategy: {}, payoff: {}, stocks_purchased: {:?}]", self.strategy, self.payoff, self.stocks_purchased)
    }
}

impl Player {

    pub fn new_blank() -> Player {
        Player {
            strategy: DataSlice::new_blank(),
            payoff: 0.0,                     // dangerous
            stocks_purchased: Vec::new(),
        }
    }

    pub fn new_uniform_random((l_limits, r_limits): (&Vec<f64>, &Vec<f64>)) -> Player {
        Player {
            strategy: DataSlice::new_uniform_random((l_limits, r_limits)),
            payoff: 0.0,                     // dangerous
            stocks_purchased: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        self.payoff = 0.0;
        self.stocks_purchased = Vec::new();
    }

    pub fn dumb_crossover(&self, player: &Player) -> Player {
        Player {
            strategy: self.strategy.dumb_crossover(&player.strategy),
            payoff: 0.0,
            stocks_purchased: Vec::new(),
        }
    }

    pub fn mutate(&self, c: f64) -> Player {
        Player {
            strategy: self.strategy.mutate(c),
            payoff: 0.0,
            stocks_purchased: Vec::new(),
        }
    }

}
