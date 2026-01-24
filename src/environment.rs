use nalgebra::DVector;

pub trait Environment: Clone + Send {
    fn observe(&self) -> &DVector<f32>;
    // The individual acts until the environment returns a final result
    fn act(&mut self, input: &DVector<f32>) -> Option<f32>;
}

#[derive(Clone)]
pub struct SteadyStateEnv {
    input: DVector<f32>,
    expected_final_output: DVector<f32>,
    steps: usize,
    step: usize,
}
impl SteadyStateEnv {
    #[must_use]
    pub fn new(
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
    #[must_use]
    fn observe(&self) -> &DVector<f32> {
        &self.input
    }

    #[must_use]
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
