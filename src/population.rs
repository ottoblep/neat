use crate::data::TestSet;
use crate::individual::Individual;

pub struct Population {
    pops: Vec<Individual>,
}
impl Population {
    pub fn new<const N_IN: usize, const N_OUT: usize, const N_POP: usize>() -> Population {
        Population {
            pops: (0..N_POP)
                .map(|_| Individual::new::<N_IN, N_OUT>())
                .collect(),
        }
    }

    fn get_sorted_idxs_by_fitness(&mut self, test_data: &TestSet) -> Vec<usize> {
        let mut indexed_fitness: Vec<(usize, f32)> = self
            .pops
            .iter_mut()
            .map(|pop: &mut Individual| pop.test_steady_state(test_data))
            .enumerate()
            .collect();

        indexed_fitness.sort_unstable_by(|(_, a): &(usize, f32), (_, b): &(usize, f32)| {
            a.partial_cmp(&b).unwrap_or(std::cmp::Ordering::Equal)
        });

        indexed_fitness
            .drain(..)
            .map(|(i_a, _): (usize, f32)| i_a)
            .collect()
    }

    pub fn average_genome_size(&self) -> f32 {
        let total_size: usize = self.pops.iter().map(|ind| ind.genome_size()).sum();
        total_size as f32 / self.pops.len() as f32
    }

    pub fn reproduce(&mut self, test_data: &TestSet, n_fittest_reprod: usize) -> Population {
        let mut order = self.get_sorted_idxs_by_fitness(test_data);
        let mut rng = rand::rng();
        Population {
            pops: order
                .drain(..)
                .map(|i| self.pops[i].clone())
                .take(n_fittest_reprod)
                .map(|ind| ind.reproduce(&mut rng))
                .collect(),
        }
    }

    pub fn expand(&mut self, target_size: usize) {
        self.pops
            .clone()
            .iter()
            .cycle()
            .take(target_size - self.pops.len())
            .for_each(|ind| {
                self.pops.push(ind.clone());
            });
    }
}
