pub struct Game {
    players: Vec<Player>,
    current_quarter: u64,
}

impl Game {

    pub fn reset(&mut self) {
        self.current_quarter = 0;
        for i in 0..self.players.len() {
            players[i].reset();
        }
    }

    pub fn next_quarter() {
        // Load the new quarter
        let quarter = Quarter::load_blank();    // temp
        // Grab new stocks
        for i in 0..self.players.len() {
            quarter.select_for_player(self.players[i]);
        }
        // Go to next quarter
        self.current_quarter += 1;
    }

    pub fn final_quarter() {
        // Load the final quarter
        let quarter = Quarter::load_blank();    // temp
        // Increment payoff by stock values
        for i in 0..self.players.len() {
            quarter.calc_payoff(self.players[i]);
        }
    }

}
