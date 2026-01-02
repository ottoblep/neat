pub struct Config {
    pub num_generations: usize,
    pub n_pop: usize,
    pub n_fittest_reproduce: usize,
    pub edge_mut_chance: u32,
    pub edge_mut_strength: f32,
    pub node_mut_chance: u32,
    pub steady_state_eval_steps_multiplier: usize,
}
