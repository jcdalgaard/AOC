mod readfile;
use std::io::{self};

fn create_vector(string_list: &[String]) -> Result<u32, String> {
    let size = string_list.first().unwrap().len();

    let mut gamma = String::from("");
    let mut epsilon = String::from("");

    for x in 0..size {
        let mut countOnes = 0;
        let mut countzero = 0;
        for line in string_list {
            if let Some(ch) = line.chars().nth(x) {
                if ch == '1' {
                    countOnes += 1;
                } else if ch == '0' {
                    countzero += 1
                }
            } else {
                println!("The position {} is out of bounds", x);
            }
        }
        if countOnes > countzero {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }

    let x = u32::from_str_radix(&gamma, 2).unwrap();
    println!("{}", x);
    let y = u32::from_str_radix(&epsilon, 2).unwrap();
    println!("{}", y);

    Ok((x * y))
}

fn create_vector2(string_slice: &[String]) -> Result<u32, String> {
    let size = string_slice.first().unwrap().len();

    let mut string_list: Vec<String> = string_slice.to_vec();
    let mut string_list2 = string_slice.to_vec();
    for x in 0..size {
        let mut count_ones = 0;
        let mut count_zeros = 0;

        let snapshot = string_list.clone();
        let snapshot2 = string_list2.clone();

        for line in &snapshot {
            if let Some(ch) = line.chars().nth(x) {
                if ch == '1' {
                    count_ones += 1;
                } else if ch == '0' {
                    count_zeros += 1;
                }
            } else {
                println!("The position {} is out of bounds", x);
            }
        }
        if snapshot.len() > 1 {
            if count_ones < count_zeros {
                string_list.retain(|s| s.chars().nth(x) == Some('0'));
            } else {
                string_list.retain(|s| s.chars().nth(x) == Some('1'));
            }
        }

        count_ones = 0;
        count_zeros = 0;
        for line in &snapshot2 {
            if let Some(ch) = line.chars().nth(x) {
                if ch == '1' {
                    count_ones += 1;
                } else if ch == '0' {
                    count_zeros += 1;
                }
            } else {
                println!("The position {} is out of bounds", x);
            }
        }

        if snapshot2.len() > 1 {
            if count_ones < count_zeros {
                string_list2.retain(|s| s.chars().nth(x) != Some('0'));
            } else {
                string_list2.retain(|s| s.chars().nth(x) != Some('1'));
            }
        }
    }

    let x = u32::from_str_radix(&string_list[0], 2).unwrap();
    println!("{}", x);
    let y = u32::from_str_radix(&string_list2[0], 2).unwrap();
    println!("{}", y);

    Ok((x * y))
}

fn calc(file_path: &str) -> Result<(u32), String> {
    let result = readfile::read_file(file_path).expect("Error in file read...");
    let number1 = create_vector(&result).unwrap();
    let number2 = create_vector2(&result).expect("Error");

    println!("{}", number1);
    println!("{}", number2);

    Ok((number1))
}

fn main() -> io::Result<()> {
    let path = "data.txt";
    let result = calc(&path).expect("Failed to calc");
    println!("taskone: {}", result);

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
    fn test_calc() {
        let path = "testData.txt";
        let result = calc(&path).expect("Failed to calc");

        assert_eq!(result, 198);
    }
}
