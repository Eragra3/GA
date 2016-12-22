use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;
use std::f64;
use std::str::FromStr;
use pbr::ProgressBar;
use std::time::Duration;

use city::City;

pub fn read(path_str: &str) -> Vec<City> {

    let path = Path::new(path_str);

    // just to get number of lines
    let dummyFile = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            panic!("File {} cannot be opened!\n{}", path_str, e);
        }
    };
    let dummyLines = BufReader::new(dummyFile).lines();
    let mut pb = ProgressBar::new(dummyLines.count() as u64);
    pb.set_max_refresh_rate(Some(Duration::from_secs(1)));

    // properly open file
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            panic!("File {} cannot be opened!\n{}", path_str, e);
        }
    };

    let mut read_cities = false;
    let mut cities = vec![];

    pb.format("╢▌▌░╟");

    for line in BufReader::new(file).lines() {
        let l = line.unwrap();

        if l == "EOF" {
            break;
        } else if read_cities {
            let mut values = l.split(" ");
            let name = u64::from_str(values.next().unwrap()).unwrap();
            let x = f64::from_str(values.next().unwrap()).unwrap();
            let y = f64::from_str(values.next().unwrap()).unwrap();

            let city = City::new(name, x, y);
            cities.push(city);
        } else if l == "NODE_COORD_SECTION" {
            read_cities = true;
        }

        pb.inc();
    }

    cities
}