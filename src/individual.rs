use crate::config::Config;
use crate::genome::Genome;
use nalgebra::DVector;
use rand::Rng;

pub trait Environment {
    fn observe(&self) -> &DVector<f32>;
    // The individual acts until the environment returns a final result
    fn act(&mut self, input: &DVector<f32>) -> Option<f32>;
}

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

    #[must_use]
    fn step(&mut self, inputs: &DVector<f32>) -> DVector<f32> {
        assert!(inputs.len() == self.genome.n_in);
        for i in 0..inputs.len() {
            self.state[i] = inputs[i];
        }
        self.state = &self.genome.network * &self.state;
        self.rectify();
        self.state.rows(self.genome.n_in, self.genome.n_out).into()
    }

    #[must_use]
    pub fn evaluate(&mut self, env: &mut impl Environment) -> f32 {
        loop {
            let input: &DVector<f32> = env.observe();
            let output: DVector<f32> = self.step(&input);
            match env.act(&output) {
                Some(r) => return r,
                None => continue,
            }
        }
    }

    #[must_use]
    pub fn reproduce(&self, rng_dev: &mut impl Rng, conf: &Config) -> Individual {
        let mut genome = self.genome.clone();
        for _ in 0..self.genome.size() {
            if rng_dev.random_range(0..100) < conf.edge_mut_chance {
                genome = genome.mutate_edge(conf.edge_mut_strength, rng_dev);
            }
        }
        if rng_dev.random_range(0..100) < conf.node_mut_chance {
            genome = genome.mutate_addnode(rng_dev);
        }
        if rng_dev.random_range(0..100) < conf.node_mut_chance {
            genome = genome.mutate_removenode(rng_dev);
        }
        Individual::from_genome(genome)
    }

    #[must_use]
    pub fn genome_size(&self) -> usize {
        self.genome.size()
    }

    #[must_use]
    pub fn get_genome(&self) -> &Genome {
        &self.genome
    }
}

#[cfg(test)]
mod tests {
    use super::Individual;
    use crate::data::TestSet;
    use nalgebra::dvector;

    #[test]
    fn test_from_genome() {
        const TEST_IN: usize = 4;
        const TEST_OUT: usize = 2;
        const TEST_GENOME_SIZE: usize = 8;

        let mut genome = super::Genome::new::<TEST_IN, TEST_OUT>();
        let mut rng = rand::rng();
        genome = genome.mutate_addnode(&mut rng);
        genome = genome.mutate_addnode(&mut rng);
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
        let mut genome = super::Genome::new::<2, 1>();
        let mut rng = rand::rng();
        genome = genome.mutate_addnode(&mut rng);
        genome = genome.mutate_addnode(&mut rng);
        genome = genome.mutate_addnode(&mut rng);
        let mut ind: Individual = Individual::from_genome(genome);
        ind.state = dvector![-2.0, -3.0, -4.0, 5.0, -6.0, -7.0];
        ind.rectify();
        assert_eq!(ind.state, dvector![-2.0, -3.0, 0.0, 5.0, 0.0, -7.0]);
    }

    #[test]
    fn test_nondestructive_addnode() {
        use crate::environment::eval_steady_state;
        let xor_test_inputs: TestSet = TestSet::new(
            vec![
                dvector![0.0, 0.0],
                dvector![0.0, 1.0],
                dvector![1.0, 0.0],
                dvector![1.0, 1.0],
            ],
            vec![dvector![0.0], dvector![1.0], dvector![1.0], dvector![0.0]],
        );
        let mut rng = rand::rng();
        let mut genome = super::Genome::new::<2, 1>();
        let mut ind: Individual = Individual::from_genome(genome.clone());
        let fitness_before: f32 = eval_steady_state(&mut ind, &xor_test_inputs, 20);
        let genome_mut = genome.mutate_addnode(&mut rng);
        let mut ind2: Individual = Individual::from_genome(genome_mut);
        let fitness_after: f32 = eval_steady_state(&mut ind2, &xor_test_inputs, 20);
        assert_eq!(fitness_before, fitness_after);
    }
}
