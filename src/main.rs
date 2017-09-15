#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate rand;
extern crate pbr;
extern crate gnuplot;

mod specimen;
mod evo_params;
mod city;
mod uwaterloo_reader;
mod logging;

use pbr::ProgressBar;
use gnuplot::*;
use chrono::prelude::*;

use std::time::{Duration, SystemTime};
use std::f64;
use std::io;
use std::process;

use evo_params::*;
use specimen::*;
use city::City;

fn main() {

    let config;
    println!("Loading config from '{}'", evo_params::CONFIG_PATH);
    match evo_params::load_config() {
        Ok(c) => {
            config = c;
        },
        Err(e) => {
                match e {
                (ConfigError::CantOpenFile, msg) => {
                    println!("{}", msg);
                    process::exit(-1);
                    return
                },
                (ConfigError::CantReadFile, msg) => {
                    println!("{}", msg);
                    process::exit(-1);
                },
                (ConfigError::ConfigInvalid, msg) => {
                    println!("{}", msg);
                    process::exit(-1);
                },
                (ConfigError::FileNotExisting, msg) => {
                    println!("{}", msg);
                    println!("Couldn't find config file, creating default at '{}'", evo_params::CONFIG_PATH);
                    config = evo_params::get_default();
                    evo_params::save_config(&config);
                }
            }
        }
    }

    // let cities: Vec<City> = uwaterloo_reader::read("./data/world.tsp");
    // let cities: Vec<City> = uwaterloo_reader::read("./data/lu980.tsp");
    let cities: Vec<City> = uwaterloo_reader::read("./data/dj38.tsp");

    //print map
    // plot(&cities, false);

    let evolution_params = config.evolution_params;
    // let evolution_params: EvolutionParams = EvolutionParams {
    //     generations: 3,
    //     population_count: 100,
    //     mutation_rate: 0.03,
    //     crossover_rate: 0.85,
    //     tournament_size: 2
    // };

    let now = Local::now();
    let generations_log_file_name = format!("{}.csv", now.format("%Y-%m-%d_%H-%M-%S"));
    let mut generations_logger = logging::get_csv_writer(&generations_log_file_name);

    let mut worst_specimens_fitness: Vec<f64> = Vec::with_capacity(evolution_params.population_count + 1 as usize);
    let mut avg_specimens_fitness: Vec<f64> = Vec::with_capacity(evolution_params.population_count + 1);
    let mut best_specimens_fitness: Vec<f64> = Vec::with_capacity(evolution_params.population_count + 1);

    let mut next_generation: Vec<Specimen<City>> = vec![];
    let mut current_generation: Vec<Specimen<City>> = vec![];

    for _ in 0..evolution_params.population_count {
        current_generation.push(Specimen::<City>::random(&cities));
    }

    let (mut best_fitness, mut worst_fitness, mut avg_fitness) = get_generation_fitness(&current_generation);
    
    //log
    let row = logging::GenerationRecord {
        generation: 0,
        best_fitness: best_fitness,
        avg_fitness: avg_fitness,
        worst_fitness: worst_fitness
    };
    generations_logger.serialize(row);

    let mut best_specimen: Option<Specimen<City>> = None;

    println!("");
    for generation in 0..evolution_params.generations {

        println!("generation {}\n", generation);

        let mut pb = ProgressBar::new(evolution_params.population_count as u64);
        pb.set_max_refresh_rate(Some(Duration::from_millis(250)));

        let mut i = 0;
        while i < evolution_params.population_count {
            let parent = tournament(&current_generation, &evolution_params);

            let mut new_specimen;
            if rand::random::<f64>() < evolution_params.crossover_rate {
                let mut waifu = tournament(&current_generation, &evolution_params);
                while parent == waifu {
                    // println!("Can't marry yourself ");
                    // println!("\nparent - {:?}", parent.get_names());
                    // println!("\nwaifu - {:?}", waifu.get_names());
                    waifu = tournament(&current_generation, &evolution_params);
                }
                
                new_specimen = parent.crossover(&waifu);
            } else {
                new_specimen = parent;
            }
            
            new_specimen.mutate(&evolution_params);
            
            //do not allow twins
            if next_generation.contains(&new_specimen) {
                print!("\t\tTwin!\n");
                // let pos = next_generation
                //     .iter()
                //     .position(|s| s == &new_specimen)
                //     .unwrap();
                // let names = next_generation[pos].get_names();
                // print!("\n\tGenotype - {:?}", names);
                // println!("\n New specimen genotype - {:?}", new_specimen.get_names());
                continue;
            }

            next_generation.push(new_specimen);
            pb.inc();
            i += 1;
        }
        pb.finish_println("");

        // get best specimen
        {
            match current_generation.iter().max_by_key(|v| Ordf64(v.fitness())) {
                Some(best) => {
                    println!("\tbest-fitness: \t{:?}", best.fitness());
                    best_specimen = Some((*best).clone());
                },
                None => print!("No specimen found!")
            }
            
            match get_generation_fitness(&current_generation) {
                (b, w, a) => {
                    best_fitness = b;
                    worst_fitness = w;
                    avg_fitness = a;
                }
            }

            println!("\tbest: \t{:?}", best_fitness);
            println!("\tavg: \t{:?}", avg_fitness);
            println!("\tworst: \t{:?}", worst_fitness);

            //log
            let row = logging::GenerationRecord {
                generation: generation as usize,
                best_fitness: best_fitness,
                avg_fitness: avg_fitness,
                worst_fitness: worst_fitness
            };
            generations_logger.serialize(row);
        }

        // swap generations
        std::mem::swap(&mut current_generation, &mut next_generation);
        next_generation.clear();
    }

    // print result
    match best_specimen {
        Some(specimen) => {
            // print!("\n\nSolution - {:?}", specimen);
            print!("\n\tIs valid - {:?}", specimen.is_valid());
            print!("\n\tGenotype - {:?}", specimen.get_names());
            //print solution
            plot(&(specimen.genotype.into_iter().map(|c| (*c).clone()).collect()), true);
        }
        None => print!("No solution found!"),
    }

    println!("\nPress enter to finish");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}

fn get_generation_fitness<T: CanBeEvaluated>(vector: &Vec<T>) -> (f64, f64, f64) {
    let mut worst_fitness = f64::INFINITY;
    let mut best_fitness = f64::NEG_INFINITY;
    let mut sum_fitness = 0.;
    for specimen in vector {
        let fitness = specimen.fitness();
        if fitness < worst_fitness {
            worst_fitness = fitness;
        }
        if fitness > best_fitness {
            best_fitness = fitness;
        }
        sum_fitness += fitness;
    }
    let avg_fitness = sum_fitness / (vector.len() as f64);
    (best_fitness, worst_fitness, avg_fitness)
}

fn plot(cities: &Vec<City>, plot_lines: bool) {
    let mut xs = Vec::with_capacity(cities.len());
    let mut ys = Vec::with_capacity(cities.len());
    for city in cities {
        xs.push(city.x);
        ys.push(city.y);
    }
    xs.push(cities.first().unwrap().x);
    ys.push(cities.first().unwrap().y);

    let mut figure = Figure::new();
    if plot_lines {
        figure
            .axes2d()
            .lines_points(&xs, &ys, &[Caption("Cities"), Color("red"), PointSymbol('O')]);
    } else {
        figure
            .axes2d()
            .points(&xs, &ys, &[Caption("Cities"), Color("red"), PointSymbol('O')]);
    }
    figure.show();
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
    let mut winner: &T = &specimens[*indices.iter().nth(0).unwrap()];
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
    rand::thread_rng().gen_range(0, vec.len())
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