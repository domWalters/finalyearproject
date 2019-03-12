use std::env;

pub mod data_trait;

pub mod data_record;
pub mod screener;
pub mod quarter;
pub mod quarters;
pub mod player;
pub mod game;

use crate::quarters::Quarters;
use crate::game::Game;

fn main() {
    // Defaults
    let mut population_size = 100;
    let mut generation_max = 10;
    let mut iterations = 2;
    let mut percentiles = vec![10];

    // Arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Less than 1 argument submitted.");
    }
    let mut args_iter = args.iter(); args_iter.next();

    let mut arg_pairs = Vec::new();
    let mut arg_store = args_iter.next().unwrap();
    let empty_string = "".to_string();
    for arg in args_iter {
        arg_pairs.push((arg_store, arg));
        arg_store = arg;
    }
    arg_pairs.push((arg_store, &empty_string));

    println!("Supplied argument pairs: {:?}", arg_pairs);

    for (arg_one, arg_two) in arg_pairs {
        match (&arg_one[0..arg_one.len()], &arg_two[0..arg_two.len()]) {
            ("-run", _) => run(population_size, generation_max, iterations, &percentiles),
            ("-test", _) => test(),
            ("-lambda", x) => population_size = x.parse::<usize>().unwrap(),
            ("-gen_max", x) => generation_max = x.parse::<i64>().unwrap(),
            ("-iterations", x) => iterations = x.parse::<usize>().unwrap(),
            ("-percentiles", x) => {
                let split: Vec<_> = x.split(",").collect();
                percentiles = split.iter().map(|string_percent| {
                    if string_percent.contains("[") {
                        string_percent[1..string_percent.len()].parse::<usize>().unwrap()
                    } else if string_percent.contains("]") {
                        string_percent[0..(string_percent.len() - 1)].parse::<usize>().unwrap()
                    } else {
                        string_percent.parse::<usize>().unwrap()
                    }
                }).collect();
            },
            _ => {

            }
        }
    }
}

fn run(population_size: usize, generation_max: i64, iterations: usize, percentiles: &Vec<usize>) {
    println!("Running algorithm with lambda={:?}, gen_max={:?}, iter={:?}, percentiles={:?}", population_size, generation_max, iterations, percentiles);
    let quarters = Quarters::<f64>::new_quarters_from_default_file(iterations);

    for i in 0..10 {
        for percentile in percentiles {
            let mut game = Game::<usize>::new_game(quarters.clone(), population_size, *percentile);
            game.run(generation_max, iterations, *percentile, format!("test-data/output-{}-{}.txt", i, *percentile));
        }
    }
}

fn test() {
    let read_quarters = Quarters::<f64>::new_quarters_from_default_file(1);

    let mut game = Game::<usize>::new_game(read_quarters, 1, 1);
    game.read("test-data/input.txt".to_string());
    game.perform_analytical_final_run(0);
    game.print_best();
}
