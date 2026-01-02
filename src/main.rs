mod config;
mod data;
mod genome;
mod individual;
mod population;

use config::CONFIG;
use data::TestSet;
use nalgebra::dvector;

fn main() {
    let xor_test_inputs: TestSet = TestSet {
        inputs: vec![
            dvector![0.0, 0.0],
            dvector![0.0, 1.0],
            dvector![1.0, 0.0],
            dvector![1.0, 1.0],
        ],
        outputs: vec![dvector![0.0], dvector![1.0], dvector![1.0], dvector![0.0]],
    };

    let mut pop = population::Population::new::<2, 1, { CONFIG.n_pop }>();
    for _generation in 0..CONFIG.num_generations {
        let (new_pop, population_stats) =
            pop.reproduce(&xor_test_inputs, CONFIG.n_fittest_reproduce);
        println!("Generation {_generation}:");
        population_stats.print();
        pop = new_pop;
    }
}
