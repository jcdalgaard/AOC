use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Reads a file and parses each line into an integer.
///
/// # Arguments
///
/// * `file_path` - A string slice that holds the path to the input file.
///
/// # Returns
///
/// * `io::Result<Vec<i32>>` - On success, returns a vector of integers. On failure, returns an I/O error.
fn read_numbers_from_file<P: AsRef<Path>>(file_path: P) -> io::Result<Vec<i32>> {
    let file = File::open(file_path)?;
    let lines = io::BufReader::new(file).lines();

    let numbers: Vec<i32> = lines
        .filter_map(|line| line.ok()) 
        .filter_map(|line| line.trim().parse::<i32>().ok()) 
        .collect();

    Ok(numbers)
}


fn count_increases_in_windows(numbers: &[i32]) -> usize {
    numbers.windows(2).filter(|window| window[1] > window[0]).count()
}


fn count_increases<P: AsRef<Path>>(file_path: P) -> io::Result<usize> {
    let numbers = read_numbers_from_file(file_path)?;
    let increases = count_increases_in_windows(&numbers);
    Ok(increases)
}

fn main() -> io::Result<()> {
    let path = "data.txt"; 
    let increases = count_increases(path)?;
    println!("Number of increases: {}", increases);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_increases_with_test_data() {
        let test_path = "testData.txt";
        let result = count_increases(test_path).expect("Failed to count increases");
        let expected = 7;
        assert_eq!(result, expected);
    }


}
