mod readfile;
use std::io::{self};

fn find_x_cords(cordinates: &Vec<Vec<char>>,value_to_find: char) -> Result<Vec<(i32, i32)>, String> {
    let mut x_cords: Vec<(i32, i32)> = vec![];
    for y in 0..cordinates.len() {
        for x in 0..cordinates[y].len() {
            if cordinates[y][x] == value_to_find {x_cords.push((y as i32, x as i32));}
        }
    }

    Ok(x_cords)
}

fn check_for_xmas(cordinates: &Vec<Vec<char>>, x_cords: Vec<(i32, i32)>) -> Result<i32, String> {
    let mut value = 0;

    for x in x_cords {
        if check_surround(cordinates, x.1, x.0, 0, 1) {value += 1}
        if check_surround(cordinates, x.1, x.0, 1, 1) {value += 1}
        if check_surround(cordinates, x.1, x.0, 1, 0) {value += 1}
        if check_surround(cordinates, x.1, x.0, 1, -1) {value += 1}
        if check_surround(cordinates, x.1, x.0, 0, -1) {value += 1}
        if check_surround(cordinates, x.1, x.0, -1, -1){value += 1}
        if check_surround(cordinates, x.1, x.0, -1, 0) {value += 1}
        if check_surround(cordinates, x.1, x.0, -1, 1) {value += 1}
    }

    Ok(value)
}

fn check_surround(cordinates: &Vec<Vec<char>>, x: i32, y: i32, y_inc: i32, x_inc: i32) -> bool {
    let mut xmas: String = "X".to_string();

    for i in 1..4 {
        if let Some(value) = cordinates.get((y + (i * y_inc)) as usize) {
            if let Some(value2) = value.get((x + (i * x_inc)) as usize) {
                xmas.push(value2.clone());
            } else {return false;}
        } else {return false;}
    }

    return xmas == "XMAS".to_string();
}
fn loop_a(cordinates: &Vec<Vec<char>>, a_cords: Vec<(i32, i32)>) -> Result<i32, String> {
    let mut value = 0;

    for z in a_cords {if check_mas(cordinates, z.1, z.0) {value += 1}}

    Ok(value)
}

fn check_mas(cordinates: &Vec<Vec<char>>, x: i32, y: i32) -> bool {
    let mut chars: Vec<char> = vec![];

    let cords = vec![(1, 1), (1, -1), (-1, -1), (-1, 1)];

    for z in cords {
        if let Some(value) = cordinates.get((y + z.0) as usize) {
            if let Some(value2) = value.get((x + z.1) as usize) {
                chars.push(value2.clone());
            } else {return false;}
        } else {return false;}
    }

    let count_m = chars.iter().filter(|&&c| c == 'M').count();
    let count_s = chars.iter().filter(|&&c| c == 'S').count();

    return count_m == 2 && count_s == 2 && chars[0] != chars[2] 
    
}

fn calc(file_path: &str) -> Result<(i32, i32), String> {
    let string = readfile::read_file_chars(file_path).expect("error Fetching String");
    let cords = find_x_cords(&string, 'X').expect("Error");
    let result = check_for_xmas(&string, cords).expect("Error");
    let a_cords = find_x_cords(&string, 'A').expect("Error");
    let result_2 = loop_a(&string, a_cords).expect("Error");

    Ok((result, result_2))
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

        assert_eq!(result.0, 18);
        assert_eq!(result.1, 9);
    }
}
