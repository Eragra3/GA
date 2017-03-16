use rand::{self, Rng};
use std::fs::File;
use std::path::Path;
use std::io::Write;

use evo_params::EvolutionParams;
use city::City;

#[derive(Debug, Clone, PartialEq)]
pub struct Specimen<'a> {
    genotype: Vec<&'a City>,
}

impl<'a> Specimen<'a> {
    pub fn mutate(&mut self, evolution_params: &EvolutionParams) {
        for index in 0..self.genotype.len() {
            if rand::random::<f64>() < evolution_params.mutation_rate {
                swap_random(&mut self.genotype, index)
            }
        }
    }

    pub fn crossover<'b: 'a>(&self, other: &Specimen<'b>) -> Specimen<'a> {
        let mut genotype: Vec<&City> = Vec::with_capacity(self.genotype.len());
        let cut_point = rand::thread_rng().gen_range(0, self.genotype.len());
        for i in 0..cut_point {
            genotype.push(self.genotype[i]);
        }
        for gene in &other.genotype {
            if !genotype.contains(gene) {
                genotype.push(gene);
            }

            if genotype.len() == self.genotype.len() {
                break;
            }
        }
        Specimen::new(genotype)
    }

    pub fn random(cities: &Vec<City>) -> Specimen {
        let mut indecies: Vec<usize> = (0..cities.len()).collect();
        rand::thread_rng().shuffle(&mut indecies);
        let genotype: Vec<&City> = indecies.into_iter().map(|i| &cities[i]).collect();
        Specimen::new(genotype)
    }

    pub fn new(genotype: Vec<&City>) -> Specimen {
        Specimen { genotype: genotype }
    }

    pub fn fitness(&self) -> f64 {
        let mut fitness: f64 = 0.0;
        for i in 0..self.genotype.len() - 1 {
            fitness += self.genotype[i].distance_to(self.genotype[i + 1]);
        }
        match self.genotype.last() {
            Some(last_gene) => fitness += last_gene.distance_to(self.genotype[0]),
            None => panic!("Genotype cannot be empty!"),
        }
        -fitness
    }

    pub fn dump_path(&self, path_str: &str) {
        let path = Path::new(path_str);
        let mut file = File::create(path)
            .expect(format!("Couldn't create file at \"{:?}\"", path_str).as_ref());

        for city in &self.genotype {
            file.write_all(city.dump().as_bytes());
        }
    }

    pub fn get_names(&self) -> Vec<u64> {
        self.genotype.iter().map(|c| c.name).collect()
    }

    pub fn is_valid(&self) -> bool {
        let mut sorted: Vec<u64> = self.genotype.iter().map(|c| c.name).collect();
        sorted.sort();
        for i in 0..sorted.len() - 1 {
            if sorted[i] == sorted[i + 1] {
                return false;
            }
        }
        true
    }
}

fn swap_random<T>(mutable_vector: &mut [T], index: usize) {
    let vec_len = mutable_vector.len();
    if index == 0 {
        mutable_vector.swap(0, 1);
        return;
    }
    if index == vec_len - 1 {
        mutable_vector.swap(index, index - 1);
        return;
    }

    if rand::random::<f64>() < 0.5 {
        mutable_vector.swap(index, index + 1);
    } else {
        mutable_vector.swap(index, index - 1);
    }
}