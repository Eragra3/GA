extern crate csv;

use std::fs::File;
use self::csv::{Writer, WriterBuilder};

#[derive(Debug, Deserialize, Serialize)]
pub struct GenerationRecord {
    pub generation: usize,
    pub best_fitness: f64,
    pub avg_fitness: f64,
    pub worst_fitness: f64,
    pub new_specimens: usize
}

pub fn get_csv_writer(log_path: &str) -> Writer<File> {
    WriterBuilder::new()
        .delimiter(b',')
        .from_path(log_path)
        .unwrap()
}