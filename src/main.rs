mod config;
mod data;
mod genome;
mod individual;
mod population;

use crate::config::Config;
use crate::data::TestSet;
use crate::population::Population;
use rand::Rng;

fn run_algorithm(data: &TestSet, conf: &Config, rng_dev: &mut impl Rng) {
    let mut pop = Population::new::<2, 1>(conf.n_pop);
    for _generation in 0..conf.num_generations {
        let (new_pop, population_stats) = pop.reproduce(&data, rng_dev, &conf);
        println!("Generation {_generation}:");
        population_stats.print();
        pop = new_pop;
    }
}

fn main() {
    let mut rng = rand::rng();
    let conf: Config = Config {
        num_generations: 6000,
        n_pop: 1000,
        n_fittest_reproduce: 600,
        edge_mut_chance: 70,
        edge_mut_strength: 0.05,
        node_mut_chance: 2,
        steady_state_eval_steps_multiplier: 1,
    };
    let generated_test_data = TestSet::generate(
        |input: [f32; 2]| -> [f32; 1] { [input[0] * input[1]] },
        20,
        -1.0..1.0,
        &mut rng,
    );
    generated_test_data.print();
    run_algorithm(&generated_test_data, &conf, &mut rng);
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample_xor() {
        use crate::config::Config;
        use crate::data::TestSet;
        use crate::population::Population;
        use nalgebra::dvector;

        let conf: Config = Config {
            num_generations: 1000,
            n_pop: 100,
            n_fittest_reproduce: 20,
            edge_mut_chance: 80,
            edge_mut_strength: 0.1,
            node_mut_chance: 1,
            steady_state_eval_steps_multiplier: 2,
        };
        let xor_test_inputs: TestSet = TestSet::new(
            vec![
                dvector![0.0, 0.0],
                dvector![0.0, 1.0],
                dvector![1.0, 0.0],
                dvector![1.0, 1.0],
            ],
            vec![dvector![0.0], dvector![1.0], dvector![1.0], dvector![0.0]],
        );

        for _ in 0..5 {
            let mut pop = Population::new::<2, 1>(conf.n_pop);
            let mut rng = rand::rng();
            for _generation in 0..conf.num_generations {
                let (new_pop, population_stats) = pop.reproduce(&xor_test_inputs, &mut rng, &conf);
                println!("Generation {_generation}:");
                population_stats.print();
                pop = new_pop;
                if population_stats.best_fitness < 0.0001 {
                    return;
                }
            }
        }
        assert!(false);
    }
}
