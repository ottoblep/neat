pub struct Config {
    pub num_generations: usize,
    pub n_pop: usize,
    pub n_fittest_reproduce: usize,
    pub edge_mut_chance: u32,
    pub edge_mut_strength: f32,
    pub node_mut_chance: u32,
    pub steady_state_eval_steps_multiplier: usize,
}

pub const CONFIG: Config = Config {
    num_generations: 1000,
    n_pop: 100,
    n_fittest_reproduce: 20,
    edge_mut_chance: 80,
    edge_mut_strength: 0.1,
    node_mut_chance: 1,
    steady_state_eval_steps_multiplier: 2,
};
