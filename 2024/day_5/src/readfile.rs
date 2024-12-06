use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_file<P: AsRef<Path>>(file_path: P) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let lines = io::BufReader::new(file).lines();

    let numbers: Vec<String> = lines
        .filter_map(|line| line.ok())
        .map(|line| line.trim().to_string())
        .collect();

    Ok(numbers)
}
