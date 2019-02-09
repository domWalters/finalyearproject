extern crate rand;

mod generator;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let number_of_stocks = &args[1].parse::<usize>().unwrap();
    let number_of_columns = &args[2].parse::<usize>().unwrap();
    let number_of_records = &args[3].parse::<usize>().unwrap();

    println!("Generating {:?} stocks, with {:?} columns, and {:?} records...", number_of_stocks, number_of_columns, number_of_records);
    generator::build_fake_data(*number_of_stocks, *number_of_columns, *number_of_records, 1);
}
