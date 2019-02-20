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
    let generation_max = 5;
    let iterations = 5;

    let quarters = Quarters::new_quarters_from_default_file(iterations);
    let quarters2 = quarters.create_percentile_quarters(1, quarters.expensive_training_data_analysis());
    let mut game = Game::new_game(quarters2, population_size);
    //println!("{:?}", quarters2);
    game.run(generation_max, iterations);
}
