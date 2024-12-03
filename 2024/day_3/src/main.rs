mod readfile;
use regex::Regex;
use std::{
    i128,
    io::{self},
};

fn pattern_matcher(text: String) -> Result<(i128, i128), String> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let do_postions: Vec<i32> = Regex::new(r"do\(\)")
        .unwrap()
        .find_iter(&text)
        .map(|mat| mat.start() as i32)
        .collect();

    let dont_postions: Vec<i32> = Regex::new(r"don't\(\)")
        .unwrap()
        .find_iter(&text)
        .map(|mat| mat.start() as i32)
        .collect();

    let mut value: i128 = 0;
    let mut value_two: i128 = 0;

    for cap in re.captures_iter(&text) {
        let digit_one = &cap[1].parse::<i128>().unwrap();
        let digit_two = &cap[2].parse::<i128>().unwrap();

        value += digit_one * digit_two;

        if let Some(matched) = cap.get(0) {
            let start_pos: i32 = matched.start() as i32;

            let dont = &dont_postions
                .iter()
                .filter(|&&pos| pos < start_pos)
                .max()
                .copied()
                .unwrap_or(0);

            let do_ = &do_postions
                .iter()
                .filter(|&&pos| pos < start_pos)
                .max()
                .copied()
                .unwrap_or(0);

            if &dont <= &do_ {
                value_two += digit_one * digit_two;
            }
        }
    }
    Ok((value, value_two))
}

fn calc(file_path: &str) -> Result<(i128, i128), String> {
    let string = readfile::make_one_string(file_path).expect("error Fetching String");
    let result = pattern_matcher(string).expect("Error");
    Ok(result)
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

        assert_eq!(result.0, 161);
        assert_eq!(result.1, 48);
    }
}
