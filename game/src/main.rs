extern crate rand;
extern crate csv;

pub mod data_record;
pub mod screener;
pub mod quarter;
pub mod quarters;
pub mod player;
pub mod game;

use data_record::{DataRecord, StockID};
use screener::Screener;
use quarter::Quarter;
use quarters::Quarters;
use player::Player;
use game::Game;

fn main() {
    let quarters = Quarters::new_quarters_from_default_file();
    let quarters_len = quarters.len();
    let population_size = 100;
    let generation_max = 10;

    let mut game = Game::new_game(quarters, population_size);
    for i in 0..3 {
        for _j in 0..generation_max {
            game.perform_generation(quarters_len, game::DEFAULT_TOURNEY_CONST, game::DEFAULT_MUTATION_CONST);
        }
        game.perform_analytical_final_run();
        game.recalc_fields_used();
        game.soft_reset();
        println!("Run {:?} complete!", i);
        if i == 0 {
            game.ratio = 0.95;
        } else if i == 1 {
            game.ratio = 0.99;
        }
    }

}
