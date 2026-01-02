use crate::data::TestSet;
use crate::genome::Genome;
use nalgebra::DVector;
use rand::Rng;

const EDGE_MUT_CHANCE: u32 = 50;
const EDGE_MUT_STRENGTH: f32 = 0.1;
const NODE_MUT_CHANCE: u32 = 20;
const STEADY_STATE_EVAL_STEPS_MULTIPLIER: usize = 2;

#[derive(Clone)]
pub struct Individual {
    genome: Genome,
    state: DVector<f32>,
}
impl Individual {
    pub fn new<const N_IN: usize, const N_OUT: usize>() -> Individual {
        let genome = Genome::new::<N_IN, N_OUT>();
        Individual {
            state: DVector::<f32>::zeros(genome.size()),
            genome: genome,
        }
    }

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

    fn eval_steady_state(&mut self, inputs: &DVector<f32>) -> DVector<f32> {
        for _ in 1..STEADY_STATE_EVAL_STEPS_MULTIPLIER * self.genome.size() {
            self.evaluate(inputs);
        }
        self.evaluate(inputs).into()
    }

    pub fn genome_size(&self) -> usize {
        self.genome.size()
    }

    pub fn test_steady_state(&mut self, test_data: &TestSet) -> f32 {
        test_data
            .inputs
            .iter()
            .zip(test_data.outputs.iter())
            .map(|(input, output)| -> f32 {
                (self.eval_steady_state(input) - output).norm_squared()
            })
            .sum()
    }

    pub fn reproduce<RNG: Rng>(&self, rng_dev: &mut RNG) -> Individual {
        let mut genome = self.genome.clone();
        if rng_dev.random_range(0..100) < EDGE_MUT_CHANCE {
            genome.mutate_edge(EDGE_MUT_STRENGTH);
        }
        if rng_dev.random_range(0..100) < NODE_MUT_CHANCE {
            genome.mutate_addnode();
        }
        Individual::from_genome(genome)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_rectiy() {
        use super::Individual;
        use nalgebra::dvector;

        let mut genome = super::Genome::new::<2, 1>();
        genome = genome.mutate_addnode();
        genome = genome.mutate_addnode();
        genome = genome.mutate_addnode();
        let mut ind: Individual = Individual::from_genome(genome);
        ind.state = dvector![-1.0, -1.0, -1.0, 1.0, -1.0, -1.0];
        ind.rectify();
        assert_eq!(ind.state, dvector![-1.0, -1.0, 0.0, 1.0, 0.0, -1.0]);
    }
}
