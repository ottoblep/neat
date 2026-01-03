use nalgebra::DVector;

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
    pub fn get_inputs(&self) -> &Vec<DVector<f32>> {
        &self.inputs
    }

    #[must_use]
    pub fn get_outputs(&self) -> &Vec<DVector<f32>> {
        &self.outputs
    }
}

// TODO: Auto generate dataset from function closure
// TODO: normalize data or adjust error calculation to weight evenly
