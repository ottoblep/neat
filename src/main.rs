mod data;
mod genome;
mod individual;
mod population;

use data::TestSet;
use nalgebra::dvector;

const NUM_GENERATIONS: usize = 100;
const N_POP: usize = 100;
const N_FITTEST_REPRODUCE: usize = 20;

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

    let mut pop = population::Population::new::<2, 1, N_POP>();
    for _generation in 0..NUM_GENERATIONS {
        let (mut new_pop, population_stats) = pop.reproduce(&xor_test_inputs, N_FITTEST_REPRODUCE);
        println!("Generation {_generation}:");
        population_stats.print();
        new_pop.expand(N_POP);
        pop = new_pop;
    }
}
