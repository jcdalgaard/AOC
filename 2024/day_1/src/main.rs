mod readfile;
use std::io::{self};

fn create_vectors(string_list: &[String]) -> Result<(Vec<i32>, Vec<i32>), String> {
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    for line in string_list {
        let parts: Vec<&str> = line.trim().split("   ").collect();
        list1.push(
            parts[0]
                .trim()
                .parse()
                .map_err(|_| "Error in parsing".to_string())?,
        );
        list2.push(
            parts[1]
                .trim()
                .parse()
                .map_err(|_| "Error in parsing".to_string())?,
        );
    }

    list1.sort();
    list2.sort();

    Ok((list1, list2))
}

fn calc(file_path: &str) -> Result<(i32, i32), String> {
    let result = readfile::read_file(file_path).expect("Error in file read...");
    let (vector1, vector2) = create_vectors(&result).unwrap();

    let mut sum: i32 = 0;
    let mut sum2: i32 = 0;
    for x in 0..vector1.len() {
        sum += (vector1[x] - vector2[x]).abs();
        let count = (vector2.iter().filter(|&n| *n == vector1[x]).count()) as i32;
        sum2 += (vector1[x] * count)
    }

    Ok((sum, sum2))
}

fn main() -> io::Result<()> {
    let path = "data.txt";
    let result = calc(&path).expect("Failed to calc");
    println!("taskone: {}", result.0);
    println!("tasktwo: {}", result.1);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_fileread() {
        let test_path = "testata.txt";
        readfile::read_file(test_path).expect("Failed to readFile");
    }
    #[test]
    fn test_fileread_correct_path() {
        let test_path = "testData.txt";
        let result: Vec<String> = readfile::read_file(test_path).expect("Failed to readFile");

        for line in &result {
            println!("{}", line);
        }
        assert!(result.len() > 0);
    }
    #[test]
    fn test_create_vectors() {
        let result = create_vectors(&vec![
            "1   2".to_string(),
            "2   3".to_string(),
            "3   4".to_string(),
        ]);

        assert!(result.is_ok());

        if let Ok((list1, list2)) = result {
            assert_eq!(list1, vec![1, 2, 3]);
            assert_eq!(list2, vec![2, 3, 4]);
        } else {
            panic!("Fejl...");
        }
    }

    #[test]
    fn test_calc() {
        let path = "testData.txt";
        let result = calc(&path).expect("Failed to calc");

        assert_eq!(result.0, 11);
        assert_eq!(result.1, 31);
    }
}
