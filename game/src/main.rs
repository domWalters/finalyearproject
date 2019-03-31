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
    let mut population_sizes = vec![100];
    let mut generation_maxs = vec![10];
    let mut iterations = vec![3];
    let mut percentiles = vec![10];
    let mut elitism = false;
    let mut speciation = false;

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
            ("-run", _) => run(&population_sizes, &generation_maxs, &iterations, &percentiles, &elitism, &speciation),
            ("-test", "") => test_file(&percentiles, &elitism, &speciation),
            ("-test", screener_string) => test_string(&percentiles, screener_string.to_string(), &elitism, &speciation),
            ("-lambda", x) => population_sizes = vector_from_string(x.to_string()),
            ("-gen_max", x) => generation_maxs = vector_from_string(x.to_string()),
            ("-iterations", x) => iterations = vector_from_string(x.to_string()),
            ("-percentiles", x) => percentiles = vector_from_string(x.to_string()),
            ("-elitism", _) => elitism = true,
            ("-speciation", _) => speciation = true,
            _ => {}
        }
    }
}

fn vector_from_string(string: String) -> Vec<usize> {
    let split: Vec<_> = string.split(",").collect();
    split.iter().map(|string_percent| {
        if string_percent.contains("[") & string_percent.contains("]") {
            string_percent[1..(string_percent.len() - 1)].parse::<usize>().unwrap()
        } else if string_percent.contains("[") {
            string_percent[1..string_percent.len()].parse::<usize>().unwrap()
        } else if string_percent.contains("]") {
            string_percent[0..(string_percent.len() - 1)].parse::<usize>().unwrap()
        } else {
            string_percent.parse::<usize>().unwrap()
        }
    }).collect::<Vec<usize>>()
}

fn run(population_sizes: &Vec<usize>, generation_maxs: &Vec<usize>, iterations: &Vec<usize>, percentiles: &Vec<usize>, elitism: &bool, speciation: &bool) {
    println!("Running algorithm with lambda={:?}, gen_max={:?}, iter={:?}, percentiles={:?}, elitism={}, speciation={}", population_sizes, generation_maxs, iterations, percentiles, elitism, speciation);
    println!("This is going to execute the genetic algorithm {:?} times.", 10 * population_sizes.len() * generation_maxs.len() * iterations.len() * percentiles.len());
    for i in 0..10 {
        for iteration in iterations {
            let quarters = Quarters::<f64>::new_quarters_from_default_file(*iteration);
            for population_size in population_sizes {
                for generation_max in generation_maxs {
                    for percentile in percentiles {
                        let mut game = Game::<usize>::new_game(quarters.clone(), *population_size, *percentile, *elitism, *speciation);
                        game.run(*generation_max, *iteration, *percentile, format!("test-data/output-r{}-perc{}-g{}-i{}-pop{}.txt", i, *percentile, *generation_max, *iteration, *population_size));
                    }
                }
            }
        }
    }
}

fn test_file(percentiles: &Vec<usize>, elitism: &bool, speciation: &bool) {
    println!("Running test_file with lambda=1, gen_max=N/A, iter=1, percentiles=[{:?}], elitism={}, speciation={}", percentiles[0], elitism, speciation);
    let read_quarters = Quarters::<f64>::new_quarters_from_default_file(1);

    let mut game = Game::<usize>::new_game(read_quarters, 1, percentiles[0], *elitism, *speciation);
    game.read_file("test-data/input.txt".to_string());
    game.perform_analytical_final_run(0);
    game.print_best();
}

fn test_string(percentiles: &Vec<usize>, screener_string: String, elitism: &bool, speciation: &bool) {
    println!("Running test_string with lambda=1, gen_max=N/A, iter=1, percentiles=[{:?}], string={:?}, elitism={}, speciation={}", percentiles[0], screener_string, elitism, speciation);
    let read_quarters = Quarters::<f64>::new_quarters_from_default_file(1);

    let mut game = Game::<usize>::new_game(read_quarters, 1, percentiles[0], *elitism, *speciation);
    game.read_string(screener_string, false);
    game.perform_analytical_final_run(0);
    game.print_best();
}
