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






fn count_increases<P: AsRef<Path>>(file_path: P, sliding_number:bool) -> io::Result<usize> {
    let  numbers = read_numbers_from_file(file_path)?;

    let transformed_numbers = transform_numbers(&numbers, sliding_number);

    let increases = count_increases_in_windows(&transformed_numbers);
    Ok(increases)
}




fn transform_numbers(numbers: &[i32], use_sliding_window: bool) -> Vec<i32> {
    if use_sliding_window {
        create_sliding_window_sums(numbers)
    } else {
        numbers.to_vec()
    }
}





fn create_sliding_window_sums(numbers: &[i32]) -> Vec<i32> {
    numbers
        .windows(3)
        .map(|window| window.iter().sum())
        .collect()
}





fn main() -> io::Result<()> {
    let path = "data.txt"; 
    let increases = count_increases(path,true)?;
    println!("Number of increases: {}", increases);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_increases_with_test_data() {
        let test_path = "testData.txt";
        let result = count_increases(test_path,false).expect("Failed to count increases");
        let expected = 7;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_count_increases_with_test_data_2() {
        let test_path = "testData.txt";
        let result = count_increases(test_path,true).expect("Failed to count increases");
        let expected = 5;
        assert_eq!(result, expected);
    }


}
