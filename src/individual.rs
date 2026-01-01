use crate::data::TestSet;
use crate::genome::Genome;
use nalgebra::{DVector, DVectorView};
use rand::Rng;

const EDGE_MUT_CHANCE: u32 = 50;
const EDGE_MUT_STRENGTH: f32 = 0.1;
const NODE_MUT_CHANCE: u32 = 20;

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

    fn evaluate(&mut self, inputs: &DVector<f32>) -> DVectorView<f32> {
        assert!(inputs.len() == self.genome.n_in);
        for i in 0..inputs.len() {
            self.state[i] = inputs[i];
        }
        self.state = &self.genome.network * &self.state;
        self.state.rows(self.genome.n_in + 1, self.genome.n_out)
    }

    fn eval_steady_state(&mut self, inputs: &DVector<f32>) -> DVectorView<f32> {
        for _ in 1..2 * self.genome.size() {
            self.evaluate(inputs);
        }
        self.evaluate(inputs)
    }

    pub fn reproduce(&self) -> Individual {
        let genome = self.genome.clone();
        if rand::rng().random_range(0..100) < EDGE_MUT_CHANCE {
            genome.mutate_edge(EDGE_MUT_STRENGTH);
        }
        if rand::rng().random_range(0..100) < NODE_MUT_CHANCE {
            genome.mutate_addnode();
        }
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
}
