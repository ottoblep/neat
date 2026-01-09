use crate::genome::Genome;

pub type Fitness = f32;
pub type PopulationFitness = Vec<Fitness>;
pub type PopulationOrdering = Vec<usize>;

pub struct PopulationStats {
    pub average_fitness: Fitness,
    pub best_fitness: Fitness,
    pub average_genome_size: usize,
    pub best_genome: Genome,
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
    data: Vec<PopulationStats>,
}
impl PopulationStatSeries {
    pub fn new() -> PopulationStatSeries {
        PopulationStatSeries { data: vec![] }
    }

    pub fn add(&mut self, new: PopulationStats) {
        self.data.push(new)
    }

    pub fn get_average_fitness_series(&self) -> Vec<(f64, f64)> {
        self.data
            .iter()
            .enumerate()
            .map(|(generation, stat)| (generation as f64, stat.average_fitness as f64))
            .collect()
    }
}
