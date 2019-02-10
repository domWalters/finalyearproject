mod generator;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let num_stocks = args[1].parse::<usize>().unwrap();
    let num_col = args[2].parse::<usize>().unwrap();
    let num_rec = args[3].parse::<usize>().unwrap();

    let tuple_vec = vec![1];

    println!("Generating {:?} stocks, with {:?} columns, and {:?} records...", num_stocks, num_col, num_rec);
    generator::build_fake_data(num_stocks, num_col, num_rec, tuple_vec);
}
