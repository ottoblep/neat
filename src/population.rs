struct Metric {}

pub trait Evaluable {
    fn evaluate(&self) -> Metric;
    fn encode(&self) -> Expressible;
}

struct Individual<N_IN: usize, N_OUT: usize> {}
impl Evaluable for Individual {
    fn evaluate(&self) -> Metric {}
    fn encode(&self) -> Expressible {}
}

pub struct Population {
    pops: Vec<dyn Evaluable>,
}
impl Population {
    fn reproduce(&self) -> GenePool {
        let genomes = self
            .pops
            .iter()
            .map(|ind| ind.encode())
            .collect::<Vec<dyn Expressible>>();
        GenePool { genomes }
    }
}
