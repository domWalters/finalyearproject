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
    let mut game = Game::new_game(quarters, 100);
    for _i in 0..100 {
        game.perform_generation_default();
    }
    game.perform_analytical_final_run();
}
