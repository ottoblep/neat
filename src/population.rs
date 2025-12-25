use nalgebra::DMatrix;

struct Metric {}

//     IN H1 H2 OUT
// IN  0  0  0  0
// H1  A  0  0  0
// H2  B  C  0  0
// OUT D  E  F  0
#[derive(Clone)]
struct Individual {
    network: DMatrix<f32>,
}
impl Individual {
    fn evaluate(&self) -> Metric {}
    fn mutate_edge(&mut self, strength: f32) {
        let nrows = self.network.nrows();
        let ncols = self.network.ncols();
        let i = rand::random::<usize>() % nrows;
        let j = rand::random::<usize>() % ncols;
        let change = (rand::random::<f32>() - 0.5) * 2.0 * strength;
        self.network[(i, j)] += change;
    }
    fn mutate_addnode(&self) -> Metric {}
    fn new(n_in: usize, n_out: usize) -> Individual {
        Individual {
            network: DMatrix::<f32>::zeros(n_in, n_out),
        }
    }
}

pub struct Population {
    pops: Vec<Individual>,
}
impl Population {}
