use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;
use std::f64;
use std::str::FromStr;

use city::City;

pub fn read(path_str: &str) -> Vec<City> {

    let path = Path::new(path_str);

    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            panic!("File {} cannot be opened!", path_str);
        }
    };

    let mut read_cities = false;

    let mut cities = vec![];

    for line in BufReader::new(file).lines() {
        let l = line.unwrap();

        if read_cities {
            let mut values = l.split(" ");
            let name = values.next().unwrap();
            let x = f64::from_str(values.next().unwrap()).unwrap();
            let y = f64::from_str(values.next().unwrap()).unwrap();

            let city = City::new(name, x, y);
            cities.push(city);
        }

        if l == "NODE_COORD_SECTION" {
            read_cities = true;
        }
    }

    cities
}