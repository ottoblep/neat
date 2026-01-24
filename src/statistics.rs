use bounded_integer::BoundedU8;

use crate::genome::Genome;

pub type Fitness = f32;
pub type PopulationFitness = Vec<Fitness>;
pub type PopulationOrdering = Vec<usize>;
pub type Percentage = BoundedU8<0, 100>;

pub struct PopulationStats {
    pub average_fitness: Fitness,
    pub best_fitness: Fitness,
    pub average_genome_size: usize,
    pub best_genome: Genome,
    pub approx_diversity: f32,
}
impl PopulationStats {
    pub fn print(&self) {
        println!("  Average fitness: {}", self.average_fitness);
        println!("  Best fitness: {}", self.best_fitness);
        println!("  Average genome size: {}", self.average_genome_size);
        println!("  Best Network");
        self.best_genome.print();
    }
}

pub struct EvaluationResult {
    pub sorted_idxs: PopulationOrdering,
    pub population_stats: PopulationStats,
}

pub struct PopulationStatSeries {
    pub average_fitness: Vec<Fitness>,
    pub best_fitness: Vec<Fitness>,
    pub average_genome_size: Vec<usize>,
    pub approx_diversity: Vec<f32>,
}
impl PopulationStatSeries {
    #[must_use]
    pub fn new() -> PopulationStatSeries {
        PopulationStatSeries {
            average_fitness: vec![],
            best_fitness: vec![],
            average_genome_size: vec![],
            approx_diversity: vec![],
        }
    }

    pub fn add(&mut self, new: PopulationStats) {
        self.average_fitness.push(new.average_fitness);
        self.best_fitness.push(new.best_fitness);
        self.average_genome_size.push(new.average_genome_size);
        self.approx_diversity.push(new.approx_diversity);
    }
}
