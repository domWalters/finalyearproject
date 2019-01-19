extern crate rand;
extern crate csv;

pub mod data_slice;
pub mod quarter;
pub mod player;
pub mod game;

pub mod csv_reader;

use std::error::Error;

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

    if let Err(err) = example() {
        println!("error running example: {}", err);
        std::process::exit(1);
    }
}

fn example() -> Result<(), Box<Error>> {
    let mut balance = load_file("AIRI_fudamentals_balance.csv".to_string());
    let mut calculations = load_file("AIRI_fudamentals_calculations.csv".to_string());
    let mut caseflow = load_file("AIRI_fudamentals_caseflow.csv".to_string());
    let mut price = load_file("AIRI_price.csv".to_string());

    unite_csv(price, calculations, caseflow, balance);


    /*
    {
        let headers = price.headers()?;
        println!("{:?}", headers);
    }
    for result in price.records() {
        let record = result?;
        println!("{:?}", record);
    }
    */
    Ok(())
}
