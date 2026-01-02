mod config;
mod data;
mod genome;
mod individual;
mod population;

fn main() {}

#[cfg(test)]
mod tests {
    #[test]
    fn test_main() {
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
        let xor_test_inputs: TestSet = TestSet {
            inputs: vec![
                dvector![0.0, 0.0],
                dvector![0.0, 1.0],
                dvector![1.0, 0.0],
                dvector![1.0, 1.0],
            ],
            outputs: vec![dvector![0.0], dvector![1.0], dvector![1.0], dvector![0.0]],
        };

        let mut pop = Population::new::<2, 1>(conf.n_pop);
        for _generation in 0..conf.num_generations {
            let (new_pop, population_stats) = pop.reproduce(&xor_test_inputs, &conf);
            println!("Generation {_generation}:");
            population_stats.print();
            pop = new_pop;
            if population_stats.best_fitness < 0.0001 {
                return;
            }
        }
        assert!(false);
    }
}
