pub struct EvolutionParams {
    pub generations: i64,
    pub population_count: i64,
    pub mutation_rate: f64,
    pub crossover_rate: f64,
    pub tournament_size: usize
}

pub struct Configuration {
    pub evolution_params: EvolutionParams
}

#[allow(dead_code)]
pub fn get_default() -> Configuration {
    Configuration {
        evolution_params: EvolutionParams {
            generations: 100,
            population_count: 100,
            mutation_rate: 0.03,
            crossover_rate: 0.85,
            tournament_size: 2
        }
    }
}