use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;
use std::f64;
use std::str::FromStr;
use pbr::ProgressBar;

use city::City;

static REFRESH_RATE: u64 = 1000;

pub fn read(path_str: &str) -> Vec<City> {

    let path = Path::new(path_str);

    // just to get number of lines
    let dummy_file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            panic!("File {} cannot be opened!\n{}", path_str, e);
        }
    };
    let dummy_lines = BufReader::new(dummy_file).lines();
    let mut pb = ProgressBar::new(dummy_lines.count() as u64);

    // properly open file
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            panic!("File {} cannot be opened!\n{}", path_str, e);
        }
    };

    let mut read_cities = false;
    let mut cities = vec![];


    let mut index = 0;

    for line in BufReader::new(file).lines() {
        index += 1;
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

        if index % REFRESH_RATE == 0 {
            pb.add(REFRESH_RATE);
        }
    }

    pb.finish_print(&format!("loaded {} cities", index));

    cities
}