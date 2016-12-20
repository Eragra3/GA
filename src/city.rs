use rand;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct City {
    pub name: String,
    pub x: f64,
    pub y: f64,
}

impl City {
    pub fn new(name: &str, x: f64, y: f64) -> City {
        City {
            name: String::from_str(name).unwrap(),
            x: x,
            y: y,
        }
    }

    pub fn random() -> City {
        use uuid::Uuid;
        City {
            name: Uuid::new_v4().hyphenated().to_string(),
            x: random(),
            y: random(),
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