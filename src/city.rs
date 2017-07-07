#[derive(Debug, Clone)]
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

    #[allow(dead_code)]
    pub fn dump(&self) -> String {
        format!("{} {} {}", self.name, self.x, self.y)
    }
}

impl PartialEq for City {
    fn eq(&self, other: &City) -> bool {
        self.name == other.name
    }
}

// impl PartialOrd for City {
//     fn partial_cmp(&self, other: &City) -> Option<Ordering> {
//         Some(self.name.cmp(&other.name))
//     }
// }

// impl Ord for City {
//     fn cmp(&self, other: &City) -> Ordering {
//         self.name.cmp(&other.name)
//     }
// }