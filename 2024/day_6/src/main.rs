mod readfile;
use std::io::{self};

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn move_guard(direction: &Direction) -> (i32, i32) {
    match direction {
        Direction::Up => return (-1, 0),
        Direction::Down => return (1, 0),
        Direction::Left => return (0, -1),
        Direction::Right => return (0, 1),
    }
}

fn swith_direction(direction: &Direction) -> Direction {
    match direction {
        Direction::Up => return Direction::Right,
        Direction::Down => return Direction::Left,
        Direction::Left => return Direction::Up,
        Direction::Right => return Direction::Down,
    }
}

fn move_route(chars: &Vec<Vec<char>>) -> Result<(i32, i32), String> {
    let mut cord = (0, 0);
    let mut direction = Direction::Up;
    let mut matrix = chars.clone();
    let mut value = 0;

    for (i, row) in matrix.iter().enumerate() {
        if let Some(j) = row.iter().position(|&c| c == '^') {
            cord = (i, j);
        }
    }

    let mut prev_cord = (0, 0);

    let mut run = true;
    while run {
        if let Some(value) = matrix.get(cord.0) {
            if let Some(value2) = value.get(cord.1) {
                if matrix[cord.0][cord.1] == '#' {
                    cord = prev_cord.clone();
                    direction = swith_direction(&direction)
                } else {
                    matrix[cord.0][cord.1] = 'X';
                    let cord_update = move_guard(&direction);
                    prev_cord = cord.clone();
                    cord = (
                        (cord.0 as i32 + cord_update.0) as usize,
                        (cord.1 as i32 + cord_update.1) as usize,
                    )
                }
            } else {
                run = false;
            }
        } else {
            run = false;
        }
    }

    for row in matrix.iter() {
        value += row.iter().filter(|&&c| c == 'X').count() as i32;
    }

    Ok((value, 1))
}

// Good ol' brute...
fn move_route_BR(chars: &Vec<Vec<char>>) -> Result<(i32, i32), String> {
    let mut cordd = (0, 0);
    let mut matrixx = chars.clone();
    let mut c = 0;

    for (i, row) in matrixx.iter().enumerate() {
        if let Some(j) = row.iter().position(|&c| c == '^') {
            cordd = (i, j);
        }
    }
    for k in 0..matrixx.len() {
        for l in 0..matrixx[k].len() {
            let mut cord = cordd.clone();
            let mut direction = Direction::Up;
            if k == cord.0 && l == cord.1 {
                continue;
            }

            let mut matrix = matrixx.clone();

            matrix[k][l] = '#';

            let mut prev_cord = (0, 0);
            let mut last_direct = false;
            let mut this_direction = true;
            let mut run = true;
            while run {
                if let Some(value) = matrix.get(cord.0) {
                    if let Some(value2) = value.get(cord.1) {
                        if matrix[cord.0][cord.1] == '#' {
                            cord = prev_cord.clone();
                            direction = swith_direction(&direction);
                            if last_direct && this_direction {
                                run = false;
                                c += 1;
                            }
                            last_direct = this_direction.clone();
                            this_direction = true;
                        } else {
                            if matrix[cord.0][cord.1] != 'X' {
                                this_direction = false;
                            }
                            matrix[cord.0][cord.1] = 'X';
                            let cord_update = move_guard(&direction);
                            prev_cord = cord.clone();
                            cord = (
                                (cord.0 as i32 + cord_update.0) as usize,
                                (cord.1 as i32 + cord_update.1) as usize,
                            )
                        }
                    } else {
                        run = false;
                    }
                } else {
                    run = false;
                }
            }
        }
    }

    Ok((c, 1))
}

fn calc(file_path: &str) -> Result<(i32, i32), String> {
    let string = readfile::read_file_chars(file_path).expect("error Fetching String");
    let result = move_route(&string).expect("error");
    let result2 = move_route_BR(&string).expect("Error");

    Ok((result.0, result2.0))
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

        assert_eq!(result.0, 41);
        assert_eq!(result.1, 6);
    }
}
