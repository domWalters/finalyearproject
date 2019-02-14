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
    let iterations = 3;

    let quarters = Quarters::new_quarters_from_default_file(iterations);
    let mut game = Game::new_game(quarters, population_size);
    game.run(generation_max, iterations);
}
