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

    fn sort_by_fitness(&mut self, test_data: &TestSet) -> Vec<usize> {
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

    pub fn reproduce<const N_POP_REPROD: usize>(&mut self, test_data: &TestSet) -> Population {
        let mut order = self.sort_by_fitness(test_data);
        let mut rng = rand::rng();
        Population {
            pops: order
                .drain(..)
                .map(|i| self.pops[i].clone())
                .take(N_POP_REPROD)
                .map(|ind| ind.reproduce(&mut rng))
                .collect(),
        }
    }
}
