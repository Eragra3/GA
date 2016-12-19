extern crate rand;

use rand::{thread_rng, Rng};

use evo_params::EvolutionParams;

#[derive(Debug)]
pub struct Specimen {
    genotype: Vec<usize>,
}

impl Specimen {
    fn mutate(&mut self, evolution_params: &EvolutionParams) {
        print!("before: {:?}", self);
        for index in 0..self.genotype.len() {
            if rand::random::<f64>() < evolution_params.mutation_rate {
                swap_random(&mut self.genotype, index)
            }
        }
        print!("after: {:?}", self);
    }

    pub fn random(evolution_params: &EvolutionParams) -> Specimen {
        let mut genotype: Vec<usize> = (0..evolution_params.genotype_length).collect();
        rand::thread_rng().shuffle(genotype.as_mut_slice());
        let specimen: Specimen = Specimen { genotype: genotype };
        specimen
    }
}

fn swap_random<T>(vector: &mut Vec<T>, index: usize) {
    let mutable_vector = vector.as_mut_slice();
    let vec_len = mutable_vector.len();
    if index == 0 {
        mutable_vector.swap(0, 1);
        return;
    }
    if index == vec_len {
        mutable_vector.swap(vec_len, vec_len - 1);
        return;
    }

    if rand::random::<f64>() < 0.5 {
        mutable_vector.swap(index, index + 1);
    } else {
        mutable_vector.swap(index, index - 1);
    }
}