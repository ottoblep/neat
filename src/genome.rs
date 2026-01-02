use nalgebra::DMatrix;
use rand::Rng;

#[derive(Clone)]
pub struct Genome {
    pub network: DMatrix<f32>,
    pub n_in: usize,
    pub n_out: usize,
}
impl Genome {
    #[must_use]
    pub fn new<const N_IN: usize, const N_OUT: usize>() -> Genome {
        Genome {
            network: DMatrix::<f32>::zeros(N_IN + N_OUT, N_IN + N_OUT),
            n_in: N_IN,
            n_out: N_OUT,
        }
    }

    #[must_use]
    fn random_idx(&self, rng_dev: &mut impl Rng) -> (usize, usize) {
        let i = rng_dev.random::<u64>() % self.network.nrows() as u64;
        let j = rng_dev.random::<u64>() % self.network.ncols() as u64;
        (i as usize, j as usize)
    }

    #[must_use]
    pub fn mutate_edge(&mut self, strength: f32, rng_dev: &mut impl Rng) -> Genome {
        let mut new = self.clone();
        let (i, j) = self.random_idx(rng_dev);
        let change = (rng_dev.random::<f32>() - 0.5) * 2.0 * strength;
        new.network[(i, j)] += change;
        new
    }

    #[must_use]
    pub fn mutate_addnode(&mut self, rng_dev: &mut impl Rng) -> Genome {
        let mut new = self.clone();
        let (i, j) = new.random_idx(rng_dev);
        let size = new.network.nrows();
        let old_weight = new.network[(i, j)];
        new.network[(i, j)] = 0.0;
        new.network.resize_mut(size + 1, size + 1, 0.0);
        new.network[(i, size)] = 1.0;
        new.network[(size, j)] = old_weight;
        new
    }

    #[must_use]
    pub fn size(&self) -> usize {
        self.network.nrows()
    }

    #[must_use]
    pub fn nodes(&self) -> usize {
        self.network.nrows() - self.n_in - self.n_out
    }

    pub fn print(&self) {
        println!("  Network weights:");
        for col in 0..self.network.ncols() {
            for row in 0..self.network.nrows() {
                print!("{:>8.3} ", self.network[(row, col)]);
            }
            println!();
        }
    }
}
