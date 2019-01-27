extern crate rand;
extern crate csv;

pub mod data_slice;
pub mod quarter;
pub mod quarters;
pub mod player;
pub mod game;

pub mod csv_reader;

use data_slice::DataSlice;
use quarter::Quarter;
use quarters::Quarters;
use player::Player;
use game::Game;

use csv_reader::csv_reader::*;

fn main() {
    create_all_unites();
    trim_and_sort();
    let quarters = Quarters::new_quarters_from_default_file();

    let mut game = Game::new_game(quarters, 100);
    for _i in 0..100 {
        game.perform_generation_default();
    }
    game.perform_analytical_final_run();
}
