mod config;
mod data;
mod environment;
mod genome;
mod individual;
mod population;
mod statistics;
mod tui;

use crate::config::Config;
use crate::data::TestSet;
use crate::population::Population;

pub fn app(terminal: &mut ratatui::DefaultTerminal) -> Result<(), std::io::Error> {
    let mut rng = rand::rng();
    let conf: Config = Config {
        num_generations: 6000,
        n_pop: 1000,
        n_fittest_reproduce: 600,
        edge_mut_chance: 70,
        edge_mut_strength: 0.1,
        node_mut_chance: 2,
        steady_state_steps: 9,
    };

    let generated_test_data = TestSet::generate(
        |input: [f32; 2]| -> [f32; 1] { [input[0] * input[1]] },
        20,
        -1.0..1.0,
        &mut rng,
    );
    // generated_test_data.print();

    let mut pop = Population::new::<2, 1>(conf.n_pop);
    let envs = generated_test_data.to_steady_state_envs(conf.steady_state_steps);

    for _generation in 0..conf.num_generations {
        let (new_pop, population_stats) = pop.reproduce(envs.iter().collect(), &mut rng, &conf);
        // println!("Generation {_generation}:");
        // population_stats.print();
        pop = new_pop;
        crate::tui::draw(terminal);
    }
    Ok(())
}

fn main() {
    let _ = ratatui::run(app);
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
            steady_state_steps: 3,
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
        let envs = xor_test_inputs.to_steady_state_envs(conf.steady_state_steps);

        for _ in 0..5 {
            let mut pop = Population::new::<2, 1>(conf.n_pop);
            let mut rng = rand::rng();
            for _generation in 0..conf.num_generations {
                let (new_pop, population_stats) =
                    pop.reproduce(envs.iter().collect(), &mut rng, &conf);
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
