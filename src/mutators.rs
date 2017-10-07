extern crate rand;

use rand::{Rng};
use rand::distributions::normal::{Normal};

#[derive(Debug)]
pub struct RandomSwap {
    
}

impl RandomSwap {
    pub fn new() -> RandomSwap {
        RandomSwap {}
    }
        
    pub fn mutate<T>(& self, mutation_rate: f64, mut value: &[T])
        where T : Copy {
        for index in 0..value.len() {
            if rand::random::<f64>() < mutation_rate {

            }
        }
    }
}