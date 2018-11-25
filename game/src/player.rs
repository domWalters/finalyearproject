use DataSlice;

pub struct Player {
    pub strategy: DataSlice,
    pub payoff: f64,
    pub stocks_purchased: Vec<DataSlice>,
}

impl Player {

    pub fn new_blank() -> Player {
        Player {
            strategy: DataSlice::new_blank(),
            payoff: 0.0,                     // dangerous
            stocks_purchased: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        self.payoff = 0.0;
        self.stocks_purchased = Vec::new();
    }

    pub fn dumb_crossover(&self, player: Player) -> Player {
        Player {
            strategy: self.strategy.dumb_crossover(player.strategy),
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
