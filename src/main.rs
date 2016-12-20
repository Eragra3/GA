extern crate uuid;
extern crate rand;

mod specimen;
mod evo_params;
mod city;
mod uwaterloo_reader;

use evo_params::EvolutionParams;
use specimen::Specimen;
use city::City;

use std::f64;

fn main() {

    let cities: Vec<City> = uwaterloo_reader::read("./data/dj38.tsp");

    // let cities: Vec<City> = (0..10).map(|_| City::random()).collect();

    let evolution_params: EvolutionParams = EvolutionParams {
        generations: 100,
        population_count: 100,
        mutation_rate: 0.05,
        crossover_rate: 0.85,
        genotype_length: cities.len(),
        tournament_size: 5,
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

        for i in 0..evolution_params.population_count {

            let parent = tournament(&current_generation, &evolution_params);

            let mut new_specimen = parent.clone();
            new_specimen.mutate(&evolution_params);
            next_generation.push(new_specimen);
        }

        // get best specimen
        {
            println!("\tbest-fitness: \t{:?}", best_specimen.fitness());

            let mut worst: &Specimen = &current_generation[0];
            let mut worst_fitness = worst.fitness();
            let mut best: &Specimen = &current_generation[0];
            let mut best_fitness = best.fitness();
            for specimen in &current_generation {
                let fitness = specimen.fitness();
                if fitness < worst_fitness {
                    worst_fitness = fitness;
                    worst = &specimen;
                }
                if fitness > best_fitness {
                    best_fitness = fitness;
                    best = &specimen;
                }
            }

            println!("\tbest: \t{:?}", best_fitness);
            println!("\tworst: \t{:?}", worst_fitness);
        }

        // swap generations
        std::mem::swap(&mut current_generation, &mut next_generation);
        next_generation.clear();
    }
}

use std::collections::HashSet;
fn tournament<'a>(specimens: &Vec<Specimen<'a>>,
                  evolution_params: &EvolutionParams)
                  -> Specimen<'a> {
    let mut indices: HashSet<usize> = HashSet::new();
    while indices.len() < evolution_params.tournament_size {
        let index = random_index(&specimens);
        if !indices.contains(&index) {
            indices.insert(index);
        }
    }
    let mut winner: &Specimen = &specimens[0];
    let mut best_fitness = winner.fitness();
    for index in indices {
        let player = &specimens[index];
        let fitness = player.fitness();
        if fitness > best_fitness {
            best_fitness = fitness;
            winner = player;
        }
    }
    return winner.clone();
}

use rand::Rng;
fn random_index<T>(vec: &Vec<T>) -> usize {
    let index = rand::thread_rng().gen_range(0, vec.len());
    return index;
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