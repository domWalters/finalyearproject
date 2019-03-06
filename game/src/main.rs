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
    let population_size = 100;
    let generation_max = 10;
    let iterations = 2;

    // Read and analyse input.txt
    let read_quarters = Quarters::<f64>::new_quarters_from_default_file(1);
    let percentiles = vec![1];

    let mut game = Game::<usize>::new_game(read_quarters, population_size, 1);
    game.read("test-data/input.txt".to_string());
    game.perform_analytical_final_run(0);
    game.print_best();

    // Run algorithm
    let quarters = Quarters::<f64>::new_quarters_from_default_file(iterations);

    for i in 0..10 {
        for percentile in &percentiles {
            let mut game = Game::<usize>::new_game(quarters.clone(), population_size, *percentile);
            game.run(generation_max, iterations, *percentile, format!("test-data/output-{}-{}.txt", i, *percentile));
        }
    }
}
