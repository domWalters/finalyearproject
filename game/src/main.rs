extern crate rand;
extern crate csv;

pub mod data_slice;
pub mod quarter;
pub mod player;
pub mod game;

pub mod csv_reader;

use data_slice::DataSlice;
use quarter::Quarter;
use player::Player;
use game::Game;

use csv_reader::csv_reader::*;

fn main() {
    let mut game = Game::new_game(100, 2);
    for _i in 0..20 {
        //println!("{}", game);
        game.perform_generation(20, 3, 0.0);
    }
    //println!("{}", game);

    unite_stock_csvs("AIRI".to_string());
    create_all_unites();
}
