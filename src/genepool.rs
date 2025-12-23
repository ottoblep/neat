pub trait Expressible {
    fn express_lossy(&self) -> Evaluable;
}

struct Genome<N_IN: usize, N_OUT: usize> {}
impl Expressible for Genome {
    fn express_lossy(&self) -> Evaluable {}
}

pub struct GenePool {
    genomes: Vec<dyn Expressible>,
}
impl GenePool {
    fn express_lossy(&self) -> Population {
        let individuals = self
            .genomes
            .iter()
            .map(|g| g.express_lossy())
            .collect::<Vec<dyn Evaluable>>();
        Population { pops: individuals }
    }
}
