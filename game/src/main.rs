pub mod data_trait;

pub mod data_record;
pub mod screener;
pub mod quarter;
pub mod quarters;
pub mod player;
pub mod game;

use crate::quarters::Quarters;
use crate::game::Game;

fn main() {
    let population_size = 100;
    let generation_max = 20;
    let iterations = 5;
    let percentile_gap = 1;

    let quarters = Quarters::<f64>::new_quarters_from_default_file(iterations);
    let mut game = Game::<usize>::new_game(quarters, population_size, percentile_gap);
    game.run(generation_max, iterations, percentile_gap);
}
