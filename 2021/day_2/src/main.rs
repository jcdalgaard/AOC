use core::num;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::string;

enum Direction {
    Forward,
    Up,
    Down,
}
fn read_numbers_from_file<P: AsRef<Path>>(file_path: P) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let lines = io::BufReader::new(file).lines();

    let numbers: Vec<String> = lines
        .filter_map(|line| line.ok())
        .filter_map(|line| line.trim().parse::<String>().ok())
        .collect();

    Ok(numbers)
}

fn parse_string(input: &String) -> Result<(Direction, i32), String> {
    let parts: Vec<&str> = input.split(' ').collect();

    if parts.len() != 2 {
        return Err("Input should contain exactly one comma".into());
    }

    let enum_value = match parts[0].trim() {
        "forward" => Direction::Forward,
        "up" => Direction::Up,
        "down" => Direction::Down,
        _ => return Err("Invalid enum value".into()),
    };

    let number: i32 = parts[1]
        .trim()
        .parse()
        .map_err(|_| "Failed to parse number".to_string())?;

    Ok((enum_value, number))
}

fn calc(file_path: &str) -> Result<i32, String> {
    let result = read_numbers_from_file(file_path).expect("Error in file read...");

    let mut x: i32 = 0;
    let mut y: i32 = 0;

    for line in result {
        println!("test : {}", line);

        match parse_string(&line) {
            Ok((enum_value, number)) => match enum_value {
                Direction::Forward => x += number,
                Direction::Down => y += number,
                Direction::Up => y -= number,
            },

            Err(err) => {
                println!("Error: {}", err);
            }
        }

        println!("x: {x}");
        println!("y: {y}");
    }

    Ok(x * y)
}

fn calc_2(file_path: &str) -> Result<i32, String> {
    let result = read_numbers_from_file(file_path).expect("Error in file read...");

    let mut x: i32 = 0;
    let mut y: i32 = 0;

    let mut aim: i32 = 0;

    for line in result {
        println!("test : {}", line);

        match parse_string(&line) {
            Ok((enum_value, number)) => match enum_value {
                Direction::Forward => {
                    x += number;
                    y += (aim * number);
                }
                Direction::Down => aim += number,
                Direction::Up => aim -= number,
            },

            Err(err) => {
                println!("Error: {}", err);
            }
        }

        println!("x: {x}");
        println!("y: {y}");
    }

    Ok(x * y)
}

fn main() -> io::Result<()> {
    let path = "data.txt";
    let result = calc(&path).expect("Failed to calc");
    println!("taskone: {}", result);
    let result2: i32 = calc_2(&path).expect("Failed to calc 2");
    println!("taskone: {}", result2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_fileread() {
        let test_path = "testata.txt";
        let result: Vec<String> = read_numbers_from_file(test_path).expect("Failed to readFile");
    }
    #[test]
    fn test_fileread_correct_path() {
        let test_path = "testData.txt";
        let result: Vec<String> = read_numbers_from_file(test_path).expect("Failed to readFile");

        for line in &result {
            println!("{}", line);
        }
        assert!(result.len() > 0);
    }
    #[test]
    fn test_calc() {
        let test_path = "testData.txt";
        let result = calc(&test_path).expect("Failed to calc");
        println!("{}", result);
        assert!(result == 150);
    }

    #[test]
    fn test_calc2() {
        let test_path = "testData.txt";
        let result = calc_2(&test_path).expect("Failed to calc");
        println!("{}", result);
        assert!(result == 900);
    }
}
