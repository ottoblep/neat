use nalgebra::{DMatrix, DVector, DVectorView, MatrixView, SVector, SVectorView};

struct Metric {}

//     IN H1 H2 OUT
// IN  0  0  0  0
// H1  A  0  0  0
// H2  B  C  0  0
// OUT D  E  F  0
// Allows recurrent connections
// Internal state, discrete time
#[derive(Clone)]
struct Genome {
    network: DMatrix<f32>,
}
impl Genome {
    pub fn mutate_edge(&mut self, strength: f32) -> Genome {
        let mut new = self.clone();
        let (i, j) = self.random_idx();
        let change = (rand::random::<f32>() - 0.5) * 2.0 * strength;
        new.network[(i, j)] += change;
        new
    }

    pub fn mutate_addnode(&mut self) -> Genome {
        let mut new = self.clone();
        let (i, j) = new.random_idx();
        let size = new.network.nrows();
        let old_weight = new.network[(i, j)];
        new.network[(i, j)] = 0.0;
        new.network.resize_mut(size + 1, size + 1, 0.0);
        new.network[(i, size + 1)] = 1.0;
        new.network[(size + 1, j)] = old_weight;
        new
    }

    pub fn new(n_in: usize, n_out: usize) -> Genome {
        Genome {
            network: DMatrix::<f32>::zeros(n_in, n_out),
        }
    }

    pub fn size(&self) -> usize {
        self.network.nrows()
    }

    fn random_idx(&self) -> (usize, usize) {
        let i = rand::random::<usize>() % self.network.nrows();
        let j = rand::random::<usize>() % self.network.ncols();
        (i, j)
    }
}

struct Individual {
    genome: Genome,
    state: DVector<f32>,
}
impl Individual {
    fn evaluate<const N_IN: usize, const N_OUT: usize>(
        &mut self,
        inputs: SVector<f32, N_IN>,
    ) -> DVectorView<f32> {
        assert!(inputs.len() + N_OUT <= self.genome.size());
        for i in 0..inputs.len() {
            self.state[i] = inputs[i];
        }
        self.state = &self.genome.network * &self.state;
        self.state.rows(self.state.nrows() - N_OUT, N_OUT)
    }

    fn new(genome: Genome) -> Individual {
        Individual {
            state: DVector::<f32>::zeros(genome.size()),
            genome: genome,
        }
    }
}

pub struct Population {
    pops: Vec<Individual>,
}
impl Population {}
