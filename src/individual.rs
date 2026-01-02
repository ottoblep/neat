use crate::config::{CONFIG, Config};
use crate::data::TestSet;
use crate::genome::Genome;
use nalgebra::DVector;
use rand::Rng;

#[derive(Clone)]
pub struct Individual {
    genome: Genome,
    state: DVector<f32>,
}
impl Individual {
    #[must_use]
    pub fn new<const N_IN: usize, const N_OUT: usize>() -> Individual {
        let genome = Genome::new::<N_IN, N_OUT>();
        Individual {
            state: DVector::<f32>::zeros(genome.size()),
            genome: genome,
        }
    }

    #[must_use]
    fn from_genome(genome: Genome) -> Individual {
        Individual {
            state: DVector::<f32>::zeros(genome.size()),
            genome: genome,
        }
    }

    fn rectify(&mut self) {
        self.state
            .iter_mut()
            .skip(self.genome.n_in)
            .take(self.genome.nodes())
            .for_each(|state: &mut f32| {
                if *state < 0.0 {
                    *state = 0.0;
                }
            });
    }

    fn evaluate(&mut self, inputs: &DVector<f32>) -> DVector<f32> {
        assert!(inputs.len() == self.genome.n_in);
        for i in 0..inputs.len() {
            self.state[i] = inputs[i];
        }
        self.state = &self.genome.network * &self.state;
        self.rectify();
        self.state.rows(self.genome.n_in, self.genome.n_out).into()
    }

    fn eval_steady_state(&mut self, inputs: &DVector<f32>, conf: &Config) -> DVector<f32> {
        for _ in 1..conf.steady_state_eval_steps_multiplier * self.genome.size() {
            self.evaluate(inputs);
        }
        self.evaluate(inputs).into()
    }

    #[must_use]
    pub fn genome_size(&self) -> usize {
        self.genome.size()
    }

    #[must_use]
    pub fn get_genome(&self) -> &Genome {
        &self.genome
    }

    #[must_use]
    pub fn test_steady_state(&mut self, test_data: &TestSet, conf: &Config) -> f32 {
        test_data
            .inputs
            .iter()
            .zip(test_data.outputs.iter())
            .map(|(input, output)| -> f32 {
                (self.eval_steady_state(input, conf) - output).norm_squared()
            })
            .sum()
    }

    #[must_use]
    pub fn reproduce<RNG: Rng>(&self, rng_dev: &mut RNG, conf: &Config) -> Individual {
        let mut genome = self.genome.clone();
        if rng_dev.random_range(0..100) < conf.edge_mut_chance {
            genome = genome.mutate_edge(conf.edge_mut_strength);
        }
        if rng_dev.random_range(0..100) < conf.node_mut_chance {
            genome = genome.mutate_addnode();
        }
        Individual::from_genome(genome)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_from_genome() {
        use super::Individual;
        const TEST_IN: usize = 4;
        const TEST_OUT: usize = 2;
        const TEST_GENOME_SIZE: usize = 8;

        let mut genome = super::Genome::new::<TEST_IN, TEST_OUT>();
        genome = genome.mutate_addnode();
        genome = genome.mutate_addnode();
        assert_eq!(genome.size(), TEST_GENOME_SIZE);
        let ind: Individual = Individual::from_genome(genome);
        assert_eq!(ind.genome.n_in, TEST_IN);
        assert_eq!(ind.genome.n_out, TEST_OUT);
        assert_eq!(ind.genome.network.nrows(), TEST_GENOME_SIZE);
        assert_eq!(ind.genome.network.ncols(), TEST_GENOME_SIZE);
        assert_eq!(ind.genome.size(), TEST_GENOME_SIZE);
        assert_eq!(ind.state.len(), TEST_GENOME_SIZE);
    }

    #[test]
    fn test_rectify() {
        use super::Individual;
        use nalgebra::dvector;

        let mut genome = super::Genome::new::<2, 1>();
        genome = genome.mutate_addnode();
        genome = genome.mutate_addnode();
        genome = genome.mutate_addnode();
        let mut ind: Individual = Individual::from_genome(genome);
        ind.state = dvector![-2.0, -3.0, -4.0, 5.0, -6.0, -7.0];
        ind.rectify();
        assert_eq!(ind.state, dvector![-2.0, -3.0, 0.0, 5.0, 0.0, -7.0]);
    }
}
