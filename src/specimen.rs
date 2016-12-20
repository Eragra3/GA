use rand::{self, thread_rng, Rng};

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

    pub fn random(cities: &Vec<City>) -> Specimen {
        let mut indecies: Vec<usize> = (0..cities.len()).collect();
        rand::thread_rng().shuffle(&mut indecies);
        let genotype: Vec<&City> = indecies.into_iter().map(|i| &cities[i]).collect();
        let specimen: Specimen = Specimen { genotype: genotype };
        specimen
    }

    pub fn fitness(&self) -> f64 {
        let mut fitness: f64 = 0.0;
        for i in 0..self.genotype.len() - 1 {
            fitness += self.genotype[i].distance_to(self.genotype[i + 1]);
        }
        -fitness
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