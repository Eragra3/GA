extern crate uuid;
extern crate rand;

mod specimen;
mod evo_params;
mod city;

use evo_params::EvolutionParams;
use specimen::Specimen;
use city::City;

fn main() {

    let cities: Vec<City> = (0..10).map(|_| City::random()).collect();

    let evolution_params: EvolutionParams = EvolutionParams {
        generations: 100,
        population_count: 100,
        mutation_rate: 0.05,
        crossover_rate: 0.85,
        genotype_length: cities.len(),
    };

    let mut solutions: Vec<Specimen> = (0..evolution_params.population_count)
        .map(|_| Specimen::random(&cities))
        .collect();

    for solution in solutions {
        print!("fitness: \t{:?}\n", solution.fitness());
    }

    for i in 0..evolution_params.generations {

    }
}