extern crate uuid;
extern crate rand;

mod specimen;
mod evo_params;

use evo_params::EvolutionParams;
use specimen::Specimen;

fn main() {

    let cities: Vec<City> = (0..10).map(|_| City::new(random(), random())).collect();

    let evolution_params: EvolutionParams = EvolutionParams {
        generations: 100,
        population_count: 100,
        mutation_rate: 0.05,
        crossover_rate: 0.85,
        genotype_length: cities.len()
    };

    let specimen = Specimen::random(&evolution_params);
    // let mut solutions: Vec<Specimen> =
    //     (0..evolution_params.population_count).map(|_| rand::random::<Specimen>()).collect();

    for i in 0..evolution_params.generations {

    }
}

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