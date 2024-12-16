use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_file_chars<P: AsRef<Path>>(file_path: P) -> io::Result<Vec<Vec<char>>> {
    let file = File::open(file_path)?;
    let lines = io::BufReader::new(file).lines();

    let numbers: Vec<Vec<char>> = lines
        .filter_map(|line| line.ok())
        .map(|line| line.trim().chars().collect())
        .collect();

    Ok(numbers)
}
