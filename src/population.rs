use crate::data::TestSet;
use crate::individual::Individual;

pub struct PopulationStats {
    average_fitness: f32,
    best_fitness: f32,
    average_genome_size: f32,
}
impl PopulationStats {
    pub fn print(&self) {
        println!("  Average fitness: {}", self.average_fitness);
        println!("  Best fitness: {}", self.best_fitness);
        println!("  Average genome size: {}", self.average_genome_size);
    }
}

struct EvaluationResult {
    sorted_idxs: Vec<usize>,
    population_stats: PopulationStats,
}

pub struct Population {
    pops: Vec<Individual>,
}
impl Population {
    #[must_use]
    pub fn new<const N_IN: usize, const N_OUT: usize, const N_POP: usize>() -> Population {
        Population {
            pops: (0..N_POP)
                .map(|_| Individual::new::<N_IN, N_OUT>())
                .collect(),
        }
    }

    #[must_use]
    fn evaluate(&mut self, test_data: &TestSet) -> EvaluationResult {
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
            .fold(f32::MAX, |a, b| a.min(b));

        indexed_fitness.sort_unstable_by(|(_, a): &(usize, f32), (_, b): &(usize, f32)| {
            a.partial_cmp(&b).unwrap_or(std::cmp::Ordering::Equal)
        });

        let sorted_idxs = indexed_fitness
            .drain(..)
            .map(|(i_a, _): (usize, f32)| i_a)
            .collect();

        EvaluationResult {
            sorted_idxs: sorted_idxs,
            population_stats: PopulationStats {
                average_fitness,
                best_fitness,
                average_genome_size: self.average_genome_size(),
            },
        }
    }

    fn expand(&mut self, target_size: usize) {
        self.pops
            .clone()
            .iter()
            .cycle()
            .take(target_size - self.pops.len())
            .for_each(|ind| {
                self.pops.push(ind.clone());
            });
    }

    #[must_use]
    pub fn average_genome_size(&self) -> f32 {
        let total_size: usize = self.pops.iter().map(|ind| ind.genome_size()).sum();
        total_size as f32 / self.pops.len() as f32
    }

    #[must_use]
    pub fn reproduce(
        &mut self,
        test_data: &TestSet,
        n_fittest_reprod: usize,
    ) -> (Population, PopulationStats) {
        let eval_result: EvaluationResult = self.evaluate(test_data);
        let mut rng = rand::rng();
        let mut pop = Population {
            pops: eval_result
                .sorted_idxs
                .iter()
                .map(|i: &usize| self.pops[*i].clone())
                .take(n_fittest_reprod)
                .map(|ind| ind.reproduce(&mut rng))
                .collect(),
        };
        pop.expand(self.pops.len());
        (pop, eval_result.population_stats)
    }
}
