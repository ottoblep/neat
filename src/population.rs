use nalgebra::{DMatrix, DVector, DVectorView, SVector};

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
    n_in: usize,
    n_out: usize,
}
impl Genome {
    pub fn new<const N_IN: usize, const N_OUT: usize>() -> Genome {
        Genome {
            network: DMatrix::<f32>::zeros(N_IN + N_OUT, N_IN + N_OUT),
            n_in: N_IN,
            n_out: N_OUT,
        }
    }

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
        new.network[(i, size)] = 1.0;
        new.network[(size, j)] = old_weight;
        new
    }

    pub fn size(&self) -> usize {
        self.network.nrows()
    }

    fn random_idx(&self) -> (usize, usize) {
        let i = rand::random::<u64>() % self.network.nrows() as u64;
        let j = rand::random::<u64>() % self.network.ncols() as u64;
        (i as usize, j as usize)
    }
}

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

    fn evaluate(&mut self, inputs: DVector<f32>) -> DVectorView<f32> {
        assert!(inputs.len() == self.genome.n_in);
        for i in 0..inputs.len() {
            self.state[i] = inputs[i];
        }
        self.state = &self.genome.network * &self.state;
        self.state
            .rows(self.state.nrows() - self.genome.n_out, self.genome.n_out)
    }

    fn eval_steady_state(&mut self, inputs: DVector<f32>) -> DVectorView<f32> {
        for _ in 1..2 * self.genome.size() {
            self.evaluate(inputs.clone());
        }
        self.evaluate(inputs)
    }
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
}
