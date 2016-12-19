extern crate uuid;
extern crate rand;

mod specimen;
mod evo_params;

use evo_params::EvolutionParams;

fn main() {
    let evolution_params: EvolutionParams = EvolutionParams {
        generations: 100,
        population_count: 100,
        mutation_rate: 0.05,
        crossover_rate: 0.85,
    };

    let cities: Vec<City> = (0..10).map(|_| City::new(random(), random())).collect();

    let mut solutions: Vec<Vec<usize>> = vec![];
    solutions.push(vec![]);

    for i in 0..evolution_params.generations {

    }
}


fn mutate() {}

fn random() -> f64 {
    rand::random::<f64>() * 100.0
}

#[derive(Debug)]
struct City {
    name: String,
    x: f64,
    y: f64,
}

impl City {
    fn new(x: f64, y: f64) -> City {
        use uuid::Uuid;
        City {
            name: Uuid::new_v4().hyphenated().to_string(),
            x: x,
            y: y,
        }
    }
}