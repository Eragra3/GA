extern crate csv;

use std::fs::File;
use std::io::prelude::*;
use self::csv::{Writer, WriterBuilder};

#[derive(Debug, Deserialize, Serialize)]
pub struct GenerationRecord {
    pub generation: usize,
    // pub best_specimen_id: String,
    // pub worst_specimen_id: String,
    pub best_fitness: f64,
    pub avg_fitness: f64,
    pub worst_fitness: f64
}

pub fn get_csv_writer(log_path: &str) -> Writer<File> {
    WriterBuilder::new()
        .delimiter(b',')
        .from_path(log_path).unwrap()
}