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
    fn express_lossy(&self) -> Population {}
}
