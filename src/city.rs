use rand;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct City {
    pub name: u64,
    pub x: f64,
    pub y: f64,
}

impl City {
    pub fn new(index: u64, x: f64, y: f64) -> City {
        City {
            name: index,
            x: x,
            y: y,
        }
    }

    pub fn distance_to(&self, other: &City) -> f64 {
        let mut x = self.x - other.x;
        x *= x;
        let mut y = self.y - other.y;
        y *= y;
        (x + y).sqrt()
    }

    pub fn dump(&self) -> String {
        format!("{} {} {}", self.name, self.x, self.y)
    }
}

fn random() -> f64 {
    rand::random::<f64>() * 100.0
}