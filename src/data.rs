use nalgebra::DVector;
use rand::Rng;
use std::ops::Range;

pub struct TestSet {
    inputs: Vec<DVector<f32>>,
    outputs: Vec<DVector<f32>>,
}
impl TestSet {
    #[must_use]
    pub fn new(inputs: Vec<DVector<f32>>, outputs: Vec<DVector<f32>>) -> TestSet {
        TestSet { inputs, outputs }
    }

    #[must_use]
    pub fn generate<const N: usize, const M: usize>(
        fun: fn([f32; N]) -> [f32; M],
        n_samples: usize,
        input_range: Range<f32>,
        rng_dev: &mut impl Rng,
    ) -> TestSet {
        let mut inputs: Vec<DVector<f32>> = Vec::with_capacity(n_samples);
        let mut outputs: Vec<DVector<f32>> = Vec::with_capacity(n_samples);
        for _ in 0..n_samples {
            let input_array: [f32; N] =
                std::array::from_fn(|_| rng_dev.random_range(input_range.clone()));
            let output_array = fun(input_array);
            inputs.push(DVector::from_column_slice(&input_array));
            outputs.push(DVector::from_column_slice(&output_array));
        }
        TestSet { inputs, outputs }
    }

    #[must_use]
    pub fn get_inputs(&self) -> &Vec<DVector<f32>> {
        &self.inputs
    }

    #[must_use]
    pub fn get_outputs(&self) -> &Vec<DVector<f32>> {
        &self.outputs
    }

    pub fn print(&self) {
        for (i, o) in self.inputs.iter().zip(self.outputs.iter()) {
            println!("Input: {:?} => Output: {:?}", i, o);
        }
    }
}

// TODO: normalize data or adjust error calculation to weight evenly
