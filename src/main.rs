extern crate uuid;
extern crate rand;

fn main() {
    let cities: Vec<City> = (0..10).map(|_| City::new(random(), random())).collect();

    print!("{:?}", cities);
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

use std::fmt::{Formatter, Error, Display};

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