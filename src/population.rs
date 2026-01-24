use crate::statistics::{
    EvaluationResult, Fitness, PopulationFitness, PopulationOrdering, PopulationStats,
};
use fraction::Fraction;
use fraction::ToPrimitive;
use itertools::iproduct;
use rand::Rng;
use rand::seq::IteratorRandom;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

use crate::config::Config;
use crate::environment::Environment;
use crate::individual::Individual;

pub struct Population {
    pops: Vec<Individual>,
}
impl Population {
    #[must_use]
    pub fn new<const N_IN: usize, const N_OUT: usize>(n_pop: usize) -> Population {
        Population {
            pops: (0..n_pop)
                .map(|_| Individual::new::<N_IN, N_OUT>())
                .collect(),
        }
    }

    fn determine_fitnesses_singleenv(&mut self, env: &impl Environment) -> PopulationFitness {
        (0..self.pops.len())
            .into_par_iter()
            .map_with(env.clone(), |thread_env, i| {
                self.pops[i].clone().evaluate(thread_env)
            })
            .collect()
    }

    fn determine_fitnesses_multienv(&mut self, envs: Vec<&impl Environment>) -> PopulationFitness {
        envs.iter()
            .map(|env| self.determine_fitnesses_singleenv(*env))
            .reduce(|fitnesses_env1, fitnesses_env2| {
                fitnesses_env1
                    .iter()
                    .zip(fitnesses_env2.iter())
                    .map(|(fitness_env1, fitness_env2)| fitness_env1 + fitness_env2)
                    .collect()
            })
            .unwrap()
    }

    #[must_use]
    fn evaluate(
        &mut self,
        envs: Vec<&impl Environment>,
        rng_dev: &mut impl Rng,
        conf: &Config,
    ) -> EvaluationResult {
        let pop_fitness: PopulationFitness = self.determine_fitnesses_multienv(envs);

        let average_fitness: f32 = pop_fitness.iter().sum::<f32>() / pop_fitness.len() as f32;

        let best_fitness: f32 = pop_fitness.iter().fold(f32::MAX, |a, b| a.min(*b));

        let mut ordering: Vec<(usize, &Fitness)> = pop_fitness.iter().enumerate().collect();
        ordering.sort_unstable_by(|(_, a): &(usize, &f32), (_, b): &(usize, &f32)| {
            a.partial_cmp(&b).unwrap_or(std::cmp::Ordering::Equal)
        });

        let sorted_idxs: PopulationOrdering = ordering
            .drain(..)
            .map(|(i_a, _): (usize, &f32)| i_a)
            .collect();

        EvaluationResult {
            population_stats: PopulationStats {
                best_genome: self.pops[sorted_idxs[0]].get_genome().clone(),
                average_fitness,
                best_fitness,
                average_genome_size: self.average_genome_size(),
                approx_diversity: self.get_approx_diversity(conf.eval_diversity_fraction, rng_dev),
            },
            sorted_idxs: sorted_idxs,
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
    pub fn average_genome_size(&self) -> usize {
        let total_size: usize = self.pops.iter().map(|ind| ind.genome_size()).sum();
        total_size / self.pops.len()
    }

    #[must_use]
    pub fn reproduce(
        &mut self,
        envs: Vec<&impl Environment>,
        rng_dev: &mut impl Rng,
        conf: &Config,
    ) -> (Population, PopulationStats) {
        let eval_result: EvaluationResult = self.evaluate(envs, rng_dev, conf);
        let mut pop = Population {
            pops: eval_result
                .sorted_idxs
                .iter()
                .map(|i: &usize| self.pops[*i].clone())
                .take(conf.n_fittest_reproduce)
                .map(|ind| ind.reproduce(rng_dev, conf))
                .collect(),
        };
        pop.expand(self.pops.len());
        (pop, eval_result.population_stats)
    }

    pub fn get_approx_diversity(
        &self,
        frac_eval_individuals: Fraction,
        rng_dev: &mut impl Rng,
    ) -> f32 {
        let n_samples: usize =
            (frac_eval_individuals.to_f64().unwrap_or(1.0) * self.pops.len() as f64) as usize;
        let sample_individuals = self.pops.iter().choose_multiple(rng_dev, n_samples);
        iproduct!(&sample_individuals, &sample_individuals)
            .map(|(ind_a, ind_b)| ind_a.genome_distance(ind_b))
            .sum::<f32>()
            / (n_samples * n_samples) as f32
    }
}
