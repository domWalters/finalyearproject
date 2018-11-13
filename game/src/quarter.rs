use DataSlice;
use Player;

pub struct Quarter {
    pub quarter_vector: Vec<DataSlice>,
}

impl Quarter {

    pub fn load_blank() -> Quarter {
        Quarter {
            quarter_vector: Vec::new()
        }
    }

    /// Returns a vector of DataSlices that are strictly larger than the input Player's strategy
    ///
    /// # Arguments
    /// * `player` - A Player object.
    ///
    /// # Remarks
    pub fn select_for_player(&self, player: Player) {
        let mut output = player.stocks_purchased;
        for i in 0..self.quarter_vector.len() {
            if self.quarter_vector[i].greater(&player.strategy) {
                output.push(self.quarter_vector[i].copy());
            }
        }
    }

    pub fn calc_payoffs(&self, mut player: Player, index: usize) {
        // For each stock the player has, find that in the quarter, and assign payoff
        for j in 0..player.stocks_purchased.len() {
            let stock = &player.stocks_purchased[j];
            match self.find_by_name(stock) {
                Some(current_value) => {
                    player.payoff += current_value.get(index) - stock.get(index)
                },
                None => println!("SERIOUS FUCKING ERROR"),  // stock you bought doesn't exist anymore
            }
        }
    }

    fn find_by_name<'a>(&self, entry: &'a DataSlice) -> Option<&'a DataSlice> {
        for i in 0..self.quarter_vector.len() {
            if self.quarter_vector[i].name == entry.name {
                return Some(entry)
            }
        }
        return None
    }

}
