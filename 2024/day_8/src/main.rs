mod readfile;
use std::collections::{HashMap, HashSet};
use std::io::{self};

fn find_antinodes_for_char(
    coord: &HashSet<(i32, i32)>,
    max_y: i32,
    max_x: i32,
) -> Result<(HashSet<(i32, i32)>, HashSet<(i32, i32)>), String> {
    let mut antinodes_for_char = HashSet::<(i32, i32)>::new();
    let mut antinodes_for_char2 = HashSet::<(i32, i32)>::new();

    for x in coord {
        for y in coord {
            if y != x {
                antinodes_for_char2.insert(*x);
                antinodes_for_char2.insert(*y);
                let diff = ((x.0 - y.0), (x.1 - y.1));
                let mut an1 = (x.0 + diff.0, x.1 + diff.1);

                if an1.0 >= 0 && an1.0 < max_y && an1.1 >= 0 && an1.1 < max_x {
                    antinodes_for_char.insert(an1.clone());
                    antinodes_for_char2.insert(an1.clone());
                }

                let mut check = true;
                while check {
                    an1 = (an1.0 + diff.0, an1.1 + diff.1);
                    if an1.0 >= 0 && an1.0 < max_y && an1.1 >= 0 && an1.1 < max_x {
                        antinodes_for_char2.insert(an1.clone());
                    } else {
                        check = false
                    }
                }
            }
        }
    }

    Ok((antinodes_for_char, antinodes_for_char2))
}

fn result(matrix: &Vec<Vec<char>>) -> Result<(i32, i32), String> {
    let mut antinodes = HashSet::<(i32, i32)>::new();
    let mut antinodes2 = HashSet::<(i32, i32)>::new();

    let mut hm: HashMap<char, HashSet<(i32, i32)>> = HashMap::new();
    let y_len = matrix.len();
    let x_len = matrix[0].len();

    for y in 0..y_len {
        for x in 0..x_len {
            if matrix[y][x] != '.' {
                hm.entry(matrix[y][x].clone())
                    .and_modify(|s| {
                        s.insert((y as i32, x as i32).clone());
                    })
                    .or_insert_with(|| {
                        let mut s = HashSet::new();
                        s.insert((y as i32, x as i32).clone());
                        s
                    });
            }
        }
    }
    for t in &hm {
        println!("{}", t.0);
        println!("HashSet = {:?}", t.1);
    }

    for x in hm.keys() {
        let found_coords =
            find_antinodes_for_char(hm.get(x).expect("Error"), y_len as i32, x_len as i32)
                .expect("Error");

        antinodes.extend(found_coords.0);
        antinodes2.extend(found_coords.1);
    }

    Ok((antinodes.len() as i32, antinodes2.len() as i32))
}

fn calc(file_path: &str) -> Result<(i32, i32), String> {
    let string = readfile::read_file_chars(file_path).expect("error Fetching String");
    let res = result(&string).expect("Errro");
    Ok((res.0, res.1))
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

        assert_eq!(result.0, 14);
        assert_eq!(result.1, 34);
    }
}
