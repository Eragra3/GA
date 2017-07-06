extern crate uuid;
extern crate rand;
extern crate pbr;

mod specimen;
mod evo_params;
mod city;
mod uwaterloo_reader;

use pbr::ProgressBar;

use std::time::Duration;
use std::f64;

use evo_params::EvolutionParams;
use specimen::*;
use city::City;

fn main() {

    static PB_BUFFER: u64 = 10;

    // let cities: Vec<City> = uwaterloo_reader::read("./data/world.tsp");
    let cities: Vec<City> = uwaterloo_reader::read("./data/lu980.tsp");
    // let cities: Vec<City> = uwaterloo_reader::read("./data/dj38.tsp");

    // let cities: Vec<City> = (0..10).map(|_| City::random()).collect();

    let evolution_params: EvolutionParams = EvolutionParams {
        generations: 100,
        population_count: 100,
        mutation_rate: 0.005,
        crossover_rate: 0.85,
        genotype_length: cities.len(),
        tournament_size: 3,
    };

    let mut next_generation: Vec<Specimen<City>> = vec![];
    let mut current_generation: Vec<Specimen<City>> = vec![];

    for _ in 0..evolution_params.population_count {
        current_generation.push(Specimen::<City>::random(&cities));
    }

    let mut best_specimen: Option<Specimen<City>> = None;

    println!("");
    for generation in 0..evolution_params.generations {
        let mut pb = ProgressBar::new(evolution_params.population_count as u64);
        pb.set_max_refresh_rate(Some(Duration::from_millis(250)));

        print!("generation {}\n", generation);

        {
            let spec_ref = current_generation.iter().max_by_key(|v| Ordf64(v.fitness())).unwrap();
            best_specimen = Some((*spec_ref).clone());
        }

        for _ in 0..evolution_params.population_count {
            let parent = tournament(&current_generation, &evolution_params);

            let mut new_specimen;
            if rand::random::<f64>() < evolution_params.crossover_rate {
                let waifu = tournament(&current_generation, &evolution_params);
                new_specimen = parent.crossover(&waifu);
            } else {
                new_specimen = parent;
            }

            new_specimen.mutate(&evolution_params);
            next_generation.push(new_specimen);

            pb.inc();
        }
        pb.finish_println("");

        // get best specimen
        {
            let mut worst: &Specimen<City> = &current_generation[0];
            let mut worst_fitness = worst.fitness();
            let mut best: &Specimen<City> = &current_generation[0];
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

            match best_specimen {
                Some(ref best) => println!("\tbest-fitness: \t{:?}", best.fitness()),
                None => print!("No specimen found!"),
            }
            println!("\tbest: \t{:?}", best_fitness);
            println!("\tworst: \t{:?}", worst_fitness);
        }

        // swap generations
        std::mem::swap(&mut current_generation, &mut next_generation);
        next_generation.clear();
    }

    // print result
    match best_specimen {
        Some(ref specimen) => {
            // print!("\n\nSolution - {:?}", specimen);
            print!("\n\tIs valid - {:?}", specimen.is_valid());
            print!("\n\tGenotype - {:?}", specimen.get_names());
        }
        None => print!("No solution found!"),
    }

}

use std::collections::HashSet;
fn tournament<T: CanBeEvaluated + Clone>(specimens: &Vec<T>,
                  evolution_params: &EvolutionParams)
                  -> T {
    let mut indices: HashSet<usize> = HashSet::new();
    while indices.len() < evolution_params.tournament_size {
        let index = random_index(&specimens);
        if !indices.contains(&index) {
            indices.insert(index);
        }
    }
    let mut winner: &T = &specimens[0];
    let mut best_fitness = winner.fitness();
    for index in indices {
        let player = &specimens[index];
        let fitness = player.fitness();
        if fitness > best_fitness {
            best_fitness = fitness;
            winner = player;
        }
    }
    return (*winner).clone();
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