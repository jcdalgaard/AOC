mod graph;
mod readfile;
use std::io::{self};
use graph::Graph;

fn create_vesc(input: Vec<String>) -> (Vec<String>, Vec<String>) {
    let mut vec1: Vec<String> = Vec::new();
    let mut vec2 = Vec::new();
    let mut bool_next = false;
    for item in input {
        if item == "" {
            bool_next = true;
        } else if !bool_next {
            vec1.push(item);
        } else {
            vec2.push(item);
        }
    }

    (vec1, vec2)
}
fn fill_graph(input: &Vec<String>, selections: &Vec<String>) -> Result<(i32, i32), String> {
    let mut result = 0;
    let mut result2 = 0;
    for x in selections {
        let mut graph = Graph::new();
        let selection: Vec<&str> = x.split(",").collect();
        for z in input {
            let potential_edges: Vec<&str> = z.split("|").collect();
            if selection.contains(&potential_edges[0]) && selection.contains(&potential_edges[1]) {
                graph.add_edge(potential_edges[0], potential_edges[1]);
            }
        }

        match graph.topological_sort() {
            Ok(order) => {
                let mut result_list = order.clone();

                result_list.retain(|element| x.contains(element));

                let res_str = &result_list.join(",").to_string();
                if res_str == x {
                    result += find_median(&result_list).expect("Error");
                } else {
                    result2 += find_median(&result_list).expect("Error");
                }
            }
            Err(e) => println!("{}", e),
        }
    }

    Ok((result, result2))
}

fn find_median(vec: &Vec<String>) -> Result<i32, String> {
    let size = vec.len();

    let median = vec[size / 2].parse::<i32>().expect("error");

    Ok(median)
}

fn calc(file_path: &str) -> Result<(i32, i32), String> {
    let string = readfile::read_file(file_path).expect("error Fetching String");
    let vecs = create_vesc(string);
    let result = fill_graph(&vecs.0, &vecs.1).expect("error");

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

        assert_eq!(result.0, 143);
        assert_eq!(result.1, 123);
    }
}
