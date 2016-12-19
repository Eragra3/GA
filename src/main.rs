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

impl Display for City {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Name: {}\n\tx: {}\n\ty: {}", self.name, self.x, self.y)
    }
}

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