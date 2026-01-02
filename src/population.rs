use crate::data::TestSet;
use crate::individual::Individual;

struct FitnessResults {
    average_fitness: f32,
    best_fitness: f32,
    sorted_indices: Vec<usize>,
}

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

    fn get_sorted_idxs_by_fitness(&mut self, test_data: &TestSet) -> FitnessResults {
        let mut indexed_fitness: Vec<(usize, f32)> = self
            .pops
            .iter_mut()
            .map(|pop: &mut Individual| pop.test_steady_state(test_data))
            .enumerate()
            .collect();

        let average_fitness: f32 = indexed_fitness
            .iter()
            .map(|(_, fit): &(usize, f32)| *fit)
            .sum::<f32>()
            / indexed_fitness.len() as f32;

        let best_fitness: f32 = indexed_fitness
            .iter()
            .map(|(_, fit): &(usize, f32)| *fit)
            .fold(f32::MIN, |a, b| a.max(b));

        indexed_fitness.sort_unstable_by(|(_, a): &(usize, f32), (_, b): &(usize, f32)| {
            a.partial_cmp(&b).unwrap_or(std::cmp::Ordering::Equal)
        });

        let sorted_idxs = indexed_fitness
            .drain(..)
            .map(|(i_a, _): (usize, f32)| i_a)
            .collect();

        FitnessResults {
            average_fitness: average_fitness,
            best_fitness: best_fitness,
            sorted_indices: sorted_idxs,
        }
    }

    pub fn average_genome_size(&self) -> f32 {
        let total_size: usize = self.pops.iter().map(|ind| ind.genome_size()).sum();
        total_size as f32 / self.pops.len() as f32
    }

    pub fn reproduce(&mut self, test_data: &TestSet, n_fittest_reprod: usize) -> Population {
        let fitness_results: FitnessResults = self.get_sorted_idxs_by_fitness(test_data);
        let mut rng = rand::rng();
        Population {
            pops: fitness_results
                .sorted_indices
                .iter()
                .map(|i: &usize| self.pops[*i].clone())
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
