use std::collections::{HashMap, VecDeque};
pub struct Graph {
    adjacent: HashMap<String, Vec<String>>,
}
// https://www.geeksforgeeks.org/topological-sorting-indegree-based-solution/
// https://hackernoon.com/topological-sorting-of-a-directed-acyclic-graph-in-rust-using-dfs-7h2c3eby
impl Graph {
    pub fn new() -> Self {
        Graph {
            adjacent: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, from: &str, to: &str) {
        self.adjacent
            .entry(from.to_string())
            .or_default()
            .push(to.to_string());
        self.adjacent.entry(to.to_string()).or_default();
    }

    pub fn topological_sort(&self) -> Result<Vec<String>, String> {
        let mut map = HashMap::new();
        for node in self.adjacent.keys() {
            map.insert(node.clone(), 0);
        }

        for edges in self.adjacent.values() {
            for edge in edges {
                *map.entry(edge.clone()).or_insert(0) += 1;
            }
        }

        let mut queue = VecDeque::new();
        for (node, &degree) in &map {
            if degree == 0 {
                queue.push_back(node.clone());
            }
        }

        let mut sorted = Vec::new();

        while let Some(node) = queue.pop_front() {
            sorted.push(node.clone());

            if let Some(edges) = self.adjacent.get(&node) {
                for edge in edges {
                    let degree = map.get_mut(edge).unwrap();
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push_back(edge.clone());
                    }
                }
            }
        }

        Ok(sorted)
    }
}
