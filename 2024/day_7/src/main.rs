mod readfile;
use itertools::Itertools;
use std::io::{self};

enum Operators {
    Multi,
    Add,
    Conca,
}

fn string_to_ope(s: &str) -> Operators {
    if s == "*" {
        return Operators::Multi;
    } else if s == "+" {
        return Operators::Add;
    } else {
        return Operators::Conca;
    }
}

fn operate(ope: &Operators, number1: &i128, number2: &i128) -> i128 {
    match ope {
        Operators::Multi => return (number1 * number2) as i128,
        Operators::Add => return (number1 + number2) as i128,
        Operators::Conca => {
            return (number1.to_string() + &number2.to_string())
                .parse::<i128>()
                .expect("Errro")
        }
    }
}
//https://users.rust-lang.org/t/how-can-i-create-a-function-with-all-permutations-of-all-digits-up-to-the-number-of-permutations-asked/75675
fn variations_up_to_length<T>(items: &[T], n: usize) -> impl Iterator<Item = Vec<&T>> {
    (n..=n).flat_map(|n| {
        std::iter::repeat(items.iter())
            .take(n)
            .multi_cartesian_product()
    })
}

fn result(list: &Vec<String>) -> Result<i128, String> {
    let mut val: i128 = 0;

    for x in list {
        val += result_one(&x).expect("errors");
    }

    Ok(val)
}
fn result_one(str: &String) -> Result<i128, String> {
    let datarow: Vec<&str> = str.split(":").collect();
    let numbers: Vec<&str> = datarow[1].trim().split(" ").collect();

    let n = numbers.len() - 1;

    // Assignment 1
    //let perm = variations_up_to_length(&["*", "+"], n);
    // Assignment 2 - ~30 sec
    let perm = variations_up_to_length(&["*", "+", "||"], n);

    for x in perm {
        let mut value: i128 = 0;
        for y in 0..x.len() {
            let op = string_to_ope(x[y]);

            if y == 0 {
                let num1 = &numbers[y].parse::<i128>().expect("error");
                let num2 = &numbers[y + 1].parse::<i128>().expect("error");

                value += operate(&op, num1, num2)
            } else {
                let num3 = &value;
                let num4 = &numbers[y + 1].parse::<i128>().expect("error");
                value = operate(&op, num3, num4)
            }
        }
        if value == datarow[0].parse::<i128>().expect("error").clone() {
            return Ok(value);
        }
    }

    Ok(0)
}

fn calc(file_path: &str) -> Result<(i128, i128), String> {
    let string = readfile::read_file(file_path).expect("error Fetching String");
    let res = result(&string).expect("Errro");
    Ok((res, 0))
}

fn main() -> io::Result<()> {
    let path = "data.txt";
    let result = calc(&path).expect("Failed to calc");
    println!("{}", result.0);
    println!("{}", result.1);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc() {
        let path = "testData.txt";
        let result = calc(&path).expect("Failed to calc");

        assert_eq!(result.0, 11387);
        assert_eq!(result.1, 0);
    }
}
