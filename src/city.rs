use rand;

#[derive(Debug)]
pub struct City {
    pub name: String,
    pub x: f64,
    pub y: f64,
}

impl City {
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
}

fn random() -> f64 {
    rand::random::<f64>() * 100.0
}