mod readfile;
use std::io::{self};

fn create_vector(string_slice: &[String]) -> Result<i32, String> {
    let string_list: Vec<String> = string_slice.to_vec();
    let mut count = 0;
    for line in string_list {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().expect("parse error"))
            .collect();
        let (increased, decreased) = validator(&numbers);

        if increased || decreased {
            count += 1;
            continue;
        }
        for x in 0..numbers.len() {
            let mut nums = numbers.clone();
            nums.remove(x);
            let (increased, decreased) = validator(&nums);
            if increased || decreased {
                count += 1;
                break;
            }
        }
    }

    Ok(count)
}

fn validator(numbers: &Vec<i32>) -> (bool, bool) {
    let mut increased = true;
    let mut decreased = true;

    for x in 1..numbers.len() {
        let var_one = numbers[x - 1];
        let var_two = numbers[x];

        if var_one < var_two {
            if var_two - var_one > 3 {
                increased = false
            }
            decreased = false;
        } else if var_one > var_two {
            if var_one - var_two > 3 {
                decreased = false
            }
            increased = false;
        } else {
            increased = false;
            decreased = false;
        }
    }
    (increased, decreased)
}

fn calc(file_path: &str) -> Result<i32, String> {
    let result = readfile::read_file(file_path).expect("Error in file read...");
    let number1 = create_vector(&result).unwrap();

    println!("{}", number1);

    Ok(number1)
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
    fn test_calc() {
        let path = "testData.txt";
        let result = calc(&path).expect("Failed to calc");

        assert_eq!(result, 4);
    }
}
