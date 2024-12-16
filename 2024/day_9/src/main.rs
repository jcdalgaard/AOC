mod readfile;
use std::io::{self};

fn result1(matrix: &Vec<char>) -> Result<(i128, i128), String> {
    let mut vec: Vec<String> = Vec::new();

    for x in 0..matrix.len() {
        let t = matrix[x].clone();
        let y = t.to_digit(10).expect("Errro");
        if x % 2 == 0 {
            for _z in 0..y {
                vec.push((x / 2).to_string());
            }
        } else {
            for _z in 0..y {
                vec.push(".".to_string());
            }
        }
    }

    let mut latest = vec.len() - 1 as usize;
    for v in 0..vec.len() {
        if vec[v] == ".".to_string() {
            if v >= latest {
                break;
            }
            let mut temp_char: String = ".".to_string();

            while temp_char == "." {
                temp_char = vec[latest].clone();
                vec[latest] = ".".to_string();
                latest = latest - 1;
            }

            vec[v] = temp_char.clone();
        }
    }

    let mut result = 0;

    for cc in 0..vec.len() {
        if vec[cc] != "." {
            result += cc as i128 * vec[cc].parse::<i128>().expect("error")
        }
    }

    Ok((result, 1))
}

fn result2(matrix: &Vec<char>) -> Result<(i128, i128), String> {
    let mut vec: Vec<Vec<String>> = Vec::new();
    let mut elements: Vec<String> = Vec::new();
    for x in 0..matrix.len() {
        let t = matrix[x].clone();
        let y = t.to_digit(10).expect("Errro");
        if y != 0 {
            if x % 2 == 0 {
                let mut temp_vec: Vec<String> = Vec::new();
                for _z in 0..y {
                    temp_vec.push((x / 2).to_string());
                }

                if y > 0 && x > 0 {
                    elements.push((x / 2).to_string());
                }

                vec.push(temp_vec);
            } else {
                let mut temp_vec: Vec<String> = Vec::new();
                for _z in 0..y {
                    temp_vec.push(".".to_string());
                }

                vec.push(temp_vec);
            }
        }
    }

    elements.reverse();
    for element in elements {
        //println!("Moving element: {}", element);
        let mut latest = 0;
        for tt in 0..vec.len() {
            if vec[tt].contains(&element) {
                latest = tt;
            }
        }

        let len_numbers = vec[latest].len();

        let mut update_rule = (0, 0, "".to_string(), false);
        for x in 0..vec.len() {
            if vec[x].contains(&".".to_string())
                && vec[x].len() >= len_numbers
                && len_numbers > 0
                && x < latest
            {
                let numbers_to_move = len_numbers;
                let char: String = element.clone();

                update_rule.0 = numbers_to_move.clone();
                update_rule.1 = x - 1.clone();
                update_rule.2 = char.to_string().clone();
                update_rule.3 = true;
                break;
            }
        }
        let add_one = update_rule.1 + 1;
        let mut vec_to_add: Vec<String> = Vec::new();
        if update_rule.3 {
            vec[latest].clear();
            for _m in 0..update_rule.0 {
                vec_to_add.push(update_rule.2.clone());
                vec[add_one].remove(0);
                vec[latest].push(".".to_string());
            }
            if vec[add_one].len() == 0 {
                vec.remove(add_one);
            }
            vec.insert(add_one, vec_to_add);
        }
    }
    let mut counter = 0;
    let mut result = 0;
    for t in vec {
        for tt in t {
            if tt != "." {
                result += counter as i128 * tt.parse::<i128>().expect("error")
            }
            counter += 1;
        }
    }

    Ok((result, 1))
}

//println!("{:?}", vec);

fn calc(file_path: &str) -> Result<(i128, i128), String> {
    let string = readfile::read_file_chars(file_path).expect("error Fetching String");
    let res = result1(&string[0]).expect("Errro");
    let res2 = result2(&string[0]).expect("Errro");
    Ok((res.0, res2.0))
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

        assert_eq!(result.0, 1928);
        assert_eq!(result.1, 2858);
    }
}
