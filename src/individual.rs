use crate::data::TestSet;
use crate::genome::Genome;
use nalgebra::{DVector, DVectorView};

pub struct Individual {
    genome: Genome,
    state: DVector<f32>,
}
impl Individual {
    pub fn new(genome: Genome) -> Individual {
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
        self.state
            .rows(self.state.nrows() - self.genome.n_out, self.genome.n_out)
    }

    fn eval_steady_state(&mut self, inputs: &DVector<f32>) -> DVectorView<f32> {
        for _ in 1..2 * self.genome.size() {
            self.evaluate(inputs);
        }
        self.evaluate(inputs)
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
