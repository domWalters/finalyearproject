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
    println!("{:?}", quarters);

    let mut game = Game::new_game(100, 2);
    for _i in 0..20 {
        game.perform_generation(20, 3, 0.0);
    }
}
