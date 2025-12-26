use crate::data::TestSet;
use crate::genome::Genome;
use crate::individual::Individual;

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

    fn sort_by_fitness(&mut self, test_data: &TestSet) -> Vec<usize> {
        let mut indexed_fitness: Vec<(usize, f32)> = self
            .pops
            .iter()
            .map(|pop| pop.test_steady_state(test_data))
            .enumerate()
            .collect();

        indexed_fitness.sort_unstable_by(|(i_a, a): &(usize, f32), (i_b, b): &(usize, f32)| {
            a.partial_cmp(&b).unwrap_or(std::cmp::Ordering::Equal)
        });

        indexed_fitness
            .drain(..)
            .map(|(i_a, _): (usize, f32)| i_a)
            .collect()
    }

    fn reproduce<const N_POP_REPROD: usize>(&mut self, test_data: &TestSet) -> Population {
        let order = self.sort_by_fitness(test_data);
        Population {
            pops: order
                .drain(..)
                .map(|i| self.pops[i])
                .take(N_POP_REPROD)
                .map(|ind| ind.reproduce())
                .collect(),
        }
    }
}
