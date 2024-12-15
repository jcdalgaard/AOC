mod readfile;
use std::collections::HashMap;
use std::io::{self};

type CacheKey = (i64, i32);
type CacheValue = i64;

enum NumberRule {
    Zero,
    EvenNumberOfDigits,
    Else,
}

fn decide_rule(number: i64) -> NumberRule {
    if number == 0 {
        return NumberRule::Zero;
    } else if number.to_string().len() % 2 == 0 {
        return NumberRule::EvenNumberOfDigits;
    } else {
        return NumberRule::Else;
    }
}

fn operation(numberrule: NumberRule, number: i64) -> Vec<i64> {
    let mut vec: Vec<i64> = Vec::new();
    match numberrule {
        NumberRule::Zero => vec.push(1),
        NumberRule::EvenNumberOfDigits => {
            let number_to_string = number.to_string();

            let len = number_to_string.chars().count();
            let mid = len / 2;

            let byte_index = number_to_string
                .char_indices()
                .nth(mid)
                .map(|(idx, _)| idx)
                .unwrap_or(number_to_string.len());

            let split = number_to_string.split_at(byte_index);

            vec.push(split.0.parse().expect("error"));
            vec.push(split.1.parse().expect("error"));
        }
        NumberRule::Else => vec.push(number * 2024),
    }

    let return_vec = vec;

    return return_vec;
}

fn get_recked(number: i64, i: i32, cache: &mut HashMap<CacheKey, CacheValue>) -> i64 {
    let key = (number, i);
    let mut value = 0;
    if let Some(&cached_result) = cache.get(&key) {
        return cached_result;
    }

    let j = i - 1;
    if j == 0 {
        cache.insert(key, 1);
        return 1;
    }

    let rule = decide_rule(number);
    let ope = operation(rule, number);

    for x in ope {
        value += get_recked(x, j, cache);
    }
    cache.insert(key, value);

    return value;
}

fn resulthm(input: String) -> Result<(i64, i64), String> {
    let inp: Vec<String> = input.split(" ").map(|s| s.to_string()).collect();

    let mut vec: Vec<i64> = Vec::new();

    let mut result1 = 0;
    let mut result2 = 0;

    let mut cache: HashMap<CacheKey, CacheValue> = HashMap::new();

    for x in inp {
        vec.push(x.clone().parse().expect("error"));
    }

    for y in 0..vec.len() {
        result1 += get_recked(vec[y], 26, &mut cache);
        result2 += get_recked(vec[y], 76, &mut cache);
    }

    Ok((result1, result2))
}

fn calc(file_path: &str) -> Result<(i64, i64), String> {
    let string = readfile::read_file(file_path).expect("error Fetching String");
    let result = resulthm(string[0].clone()).expect("error");

    Ok((result.0, result.1))
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

        assert_eq!(result.0, 55312);
        assert_eq!(result.1, 0);
    }
}
