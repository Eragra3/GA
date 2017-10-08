extern crate serde_yaml;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub static CONFIG_PATH: &'static str = "config.yaml";

#[derive(Serialize, Deserialize)]
pub struct EvolutionParams {
    pub generations: usize,
    pub population_count: usize,
    pub mutation_rate: f64,
    pub crossover_rate: f64,
    pub tournament_size: usize,
    pub allow_twins: bool
}

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    pub evolution_params: EvolutionParams,
    pub dataset_path: String,
    pub log_results: bool,
    pub version: String,
    pub log_directory: String
}

#[allow(dead_code)]
pub fn get_default() -> Configuration {
    Configuration {
        evolution_params: EvolutionParams {
            generations: 100,
            population_count: 100,
            mutation_rate: 0.03,
            crossover_rate: 0.85,
            tournament_size: 2,
            allow_twins: false
        },
        dataset_path: "".to_string(),
        log_results: true,
        version: "0.1.0".to_string(),
        log_directory: "logs/".to_string()
    }
}

#[derive(Debug)]
pub enum ConfigError {
    CantOpenFile,
    CantReadFile,
    FileNotExisting,
    ConfigInvalid
}

//todo remake to some proper struct for Result, not tuple
pub fn load_config() -> Result<Configuration, (ConfigError, String)> {
    if !Path::new(CONFIG_PATH).exists() {
        let msg = format!("File {} does not exist!", CONFIG_PATH);
        return Err((ConfigError::FileNotExisting, msg))
    }
    match File::open(CONFIG_PATH) {
        Ok(mut file) => {
            let mut content = String::new();
            match file.read_to_string(&mut content) {
                Ok(_) => {
                    match serde_yaml::from_str::<Configuration>(&content) {
                        Ok(config) => {
                            Ok(config)
                        }
                        Err(e) => {
                            let msg = format!("Cannot deserialize config at {}. File contents:\n{}", e, content);
                            Err((ConfigError::ConfigInvalid, msg))
                        }
                    }
                },
                Err(e) => {
                    let msg = format!("Can't load config from file {}, error - {}", CONFIG_PATH, e);
                    Err((ConfigError::CantReadFile, msg))
                }
            }
        },
        Err(e) => {
            let msg = format!("Can't load file at {}, error - {}", CONFIG_PATH, e);
            Err((ConfigError::CantOpenFile, msg))
        }, 
    }
}   

pub fn save_config(config: &Configuration) {
    match File::create(CONFIG_PATH) {
        Ok(mut file) => {
            let config_str = serde_yaml::to_string(&config).unwrap();
            let bytes = config_str.as_bytes();
            match file.write_all(bytes) {
                Ok(_) => { },
                Err(e) => println!("Cannot save config to {}, errror - {}", CONFIG_PATH, e)
            }
        },
        Err(e) =>  println!("Cannot create file at {}, error - {}", CONFIG_PATH, e)
    }
}