use crate::data::TestSet;
use crate::individual::Individual;
use nalgebra::DVector;

pub trait Environment {
    fn observe(&self) -> &DVector<f32>;
    // The individual acts until the environment returns a final result
    fn act(&mut self, input: &DVector<f32>) -> Option<f32>;
}

struct SteadyStateEnv {
    input: DVector<f32>,
    expected_final_output: DVector<f32>,
    steps: usize,
    step: usize,
}
impl SteadyStateEnv {
    fn new(
        input: DVector<f32>,
        expected_final_output: DVector<f32>,
        steps: usize,
    ) -> SteadyStateEnv {
        SteadyStateEnv {
            input,
            expected_final_output,
            steps,
            step: 0,
        }
    }
}
impl Environment for SteadyStateEnv {
    fn observe(&self) -> &DVector<f32> {
        &self.input
    }

    fn act(&mut self, input: &DVector<f32>) -> Option<f32> {
        if self.step < self.steps {
            self.step += 1;
            None
        } else {
            let error = (input - &self.expected_final_output).norm_squared();
            Some(error)
        }
    }
}

pub fn eval_steady_state(ind: &mut Individual, data: &TestSet, steps: usize) -> f32 {
    data.get_inputs()
        .iter()
        .zip(data.get_outputs().iter())
        .map(|(input, expected_output)| {
            let mut env = SteadyStateEnv::new(input.clone(), expected_output.clone(), steps);
            ind.evaluate(&mut env)
        })
        .sum()
}
