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
    let generation_max = 20;
    let iterations = 5;

    let quarters = Quarters::<f64>::new_quarters_from_default_file(iterations);
    let percentiles = vec![1, 2, 4, 5, 10];
    for i in 0..5 {
        for percentile in &percentiles {
            let mut game = Game::<usize>::new_game(quarters.clone(), population_size, *percentile);
            game.run(generation_max, iterations, *percentile, format!("test-data/output-{}-{}.txt", i, *percentile));
        }
    }


}
