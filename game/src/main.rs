pub mod data_record;
pub mod screener;
pub mod quarter;
pub mod quarters;
pub mod player;
pub mod game;

use crate::quarters::Quarters;
use crate::game::Game;

fn main() {
    let quarters = Quarters::new_quarters_from_default_file();
    let population_size = 100;
    let generation_max = 10;
    let prelim_iterations = 3;

    let mut game = Game::new_game(quarters, population_size);
    game.run(generation_max, prelim_iterations);
}
