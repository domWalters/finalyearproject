use DataSlice;

pub struct Player {
    pub strategy: DataSlice,
    pub payoff: f64,
    pub stocks_purchased: Vec<DataSlice>,
}

impl Player {

    pub fn reset(&mut self) {
        self.payoff = 0.0;
    }



}
