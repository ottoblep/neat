use crate::data::TestSet;
use crate::individual::{Genome, Individual};

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

    pub fn seed(mut genomes: Vec<Genome>) -> Population {
        Population {
            pops: genomes
                .drain(..)
                .map(|genome| Individual::new(genome))
                .collect(),
        }
    }

    fn reproduce(&mut self, test_data: &TestSet) -> Population {
        let fitness_values: Vec<f32> = self
            .pops
            .iter()
            .map(|pop| pop.test_steady_state(test_data))
            .collect();
    }
}
