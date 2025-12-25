use crate::individual::{Genome, Individual};
use nalgebra::DVector;

struct TestSet {
    inputs: Vec<DVector<f32>>,
    outputs: Vec<DVector<f32>>,
}

pub struct Population {
    pops: Vec<Individual>,
}
impl Population {
    pub fn new<const N_IN: usize, const N_OUT: usize, const N_POP: usize>() -> Population {
        let mut pop: Vec<Individual> = vec![];
        for _ in 0..N_POP {
            let genome = Genome::new::<N_IN, N_OUT>();
            pop.push(Individual::new(genome));
        }
        Population { pops: pop }
    }

    fn evaluate(&mut self, test_data: TestSet) {
        for individual in &mut self.pops {
            let error_sum: f32 = test_data
                .inputs
                .iter()
                .zip(test_data.outputs.iter())
                .map(|(input, target)| -> f32 {
                    let output = individual.evaluate(input);
                    (output - target).norm_squared()
                })
                .sum();
        }
    }
}
