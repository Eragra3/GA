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

    let mut next_generation: Vec<Specimen> = vec![];
    let mut current_generation: Vec<Specimen> = vec![];

    for _ in 0..evolution_params.population_count {
        current_generation.push(Specimen::random(&cities));
    }

    let mut best_specimen: Specimen;

    for generation in 0..evolution_params.generations {
        print!("generation {}\n", generation);

        {
            let spec_ref = current_generation.iter().max_by_key(|v| Ordf64(v.fitness())).unwrap();
            best_specimen = spec_ref.clone();
        }

        for specimen in &current_generation {
            let mut new_specimen = specimen.clone();
            new_specimen.mutate(&evolution_params);
            next_generation.push(new_specimen);
        }

        // get best specimen
        print!("best-fitness: \t{:?}\n", best_specimen.fitness());

        // swap generations
        std::mem::swap(&mut current_generation, &mut next_generation);
        next_generation.clear();
    }
}

use std::cmp::Ordering;
#[derive(PartialEq)]
struct Ordf64(f64);

impl PartialOrd for Ordf64 {
    fn partial_cmp(&self, other: &Ordf64) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for Ordf64 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Less)
    }
}

impl Eq for Ordf64 {}