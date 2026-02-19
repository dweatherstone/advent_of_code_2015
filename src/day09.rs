use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq)]
pub struct Graph {
    // Maps the human-readable place name to an internal index
    nodes: Vec<String>,
    names_to_index: HashMap<String, usize>,
    // Adjacency Matrix: matrix[i][j] = distance from i to j
    // Use u64::MAX if no path exists
    matrix: Vec<Vec<u64>>,
}

impl Graph {
    fn solve_shortest_tsp(&self) -> u64 {
        let n = self.nodes.len();
        let num_states = 1 << n;
        let inf = u64::MAX / 2; // Safe infinity
        // memo[mask][last_node]
        let mut memo = vec![vec![inf; n]; num_states];
        // Base case: starting at any node has 0 cost
        for i in 0..n {
            memo[1 << i][i] = 0;
        }

        // Iterate through all subsets of nodes
        for mask in 1..num_states {
            for u in 0..n {
                if memo[mask][u] == inf {
                    continue;
                }

                // Try moving to a next node 'v'
                for v in 0..n {
                    if (mask & (1 << v)) == 0 {
                        // If v is not visited
                        let next_mask = mask | (1 << v);
                        let new_dist = memo[mask][u] + self.matrix[u][v];
                        if new_dist < memo[next_mask][v] {
                            memo[next_mask][v] = new_dist;
                        }
                    }
                }
            }
        }

        // Find the minimum distance in the "all visited" row
        let final_mask = num_states - 1;
        let min_dist = memo[final_mask].iter().min().unwrap();
        *min_dist
    }

    fn solve_longest_tsp(&self) -> u64 {
        let n = self.nodes.len();
        let num_states = 1 << n;
        // memo[mask][last_node]
        let mut memo = vec![vec![0u64; n]; num_states];
        // Base case: starting at any node has 0 cost
        for i in 0..n {
            memo[1 << i][i] = 0;
        }

        // Iterate through all subsets of nodes
        for mask in 1..num_states {
            for u in 0..n {
                // Only proceed if this state is reachable
                // i.e. the mask actually contains the node u
                if (mask & (1 << u)) == 0 {
                    continue;
                }

                // Try moving to a next node 'v'
                for v in 0..n {
                    if (mask & (1 << v)) == 0 {
                        // If v is not visited
                        let next_mask = mask | (1 << v);
                        let new_dist = memo[mask][u] + self.matrix[u][v];
                        if new_dist > memo[next_mask][v] {
                            memo[next_mask][v] = new_dist;
                        }
                    }
                }
            }
        }

        // Find the minimum distance in the "all visited" row
        let final_mask = num_states - 1;
        let max_dist = memo[final_mask].iter().max().unwrap();
        *max_dist
    }
}

pub fn parse_day09(lines: &[String]) -> Graph {
    // Get all the distinct names into a hashset
    let mut names: HashSet<String> = HashSet::new();
    for line in lines {
        let (name1, rest) = line.split_once(" to ").unwrap();
        names.insert(name1.to_string());
        let (name2, _) = rest.split_once(" = ").unwrap();
        names.insert(name2.to_string());
    }
    let mut nodes: Vec<String> = Vec::from_iter(names);
    // Sort nodes into alphabetical order for easier debugging/testing
    nodes.sort();
    let names_to_index: HashMap<_, _> = nodes
        .iter()
        .enumerate()
        .map(|(idx, name)| (name.clone(), idx))
        .collect();
    // Second time through the input to map the weights of the graph.
    let mut matrix = vec![vec![u64::MAX; nodes.len()]; nodes.len()];
    for line in lines {
        let (names, distance) = line.split_once(" = ").unwrap();
        let distance = distance.parse::<u64>().unwrap();
        let (name1, name2) = names.split_once(" to ").unwrap();
        let name1_idx = names_to_index.get(name1).unwrap();
        let name2_idx = names_to_index.get(name2).unwrap();
        matrix[*name1_idx][*name2_idx] = distance;
        matrix[*name2_idx][*name1_idx] = distance;
    }
    Graph {
        nodes,
        names_to_index,
        matrix,
    }
}

pub fn result_day09_stage1(graph: &Graph) -> u64 {
    graph.solve_shortest_tsp()
}

pub fn result_day09_stage2(graph: &Graph) -> u64 {
    graph.solve_longest_tsp()
}

#[cfg(test)]
mod day09 {
    use super::*;

    fn get_example() -> Vec<String> {
        vec![
            String::from("London to Dublin = 464"),
            String::from("London to Belfast = 518"),
            String::from("Dublin to Belfast = 141"),
        ]
    }

    #[test]
    fn parse() {
        let expected_graph = Graph {
            nodes: vec![
                String::from("Belfast"),
                String::from("Dublin"),
                String::from("London"),
            ],
            names_to_index: HashMap::from([
                ("Belfast".to_string(), 0),
                ("Dublin".to_string(), 1),
                ("London".to_string(), 2),
            ]),
            matrix: vec![
                vec![u64::MAX, 141, 518],
                vec![141, u64::MAX, 464],
                vec![518, 464, u64::MAX],
            ],
        };
        let graph = parse_day09(&get_example());
        assert_eq!(graph, expected_graph);
    }

    #[test]
    fn stage1() {
        let graph = parse_day09(&get_example());
        let result = result_day09_stage1(&graph);
        assert_eq!(result, 605);
    }

    #[test]
    fn stage2() {
        let graph = parse_day09(&get_example());
        let result = result_day09_stage2(&graph);
        assert_eq!(result, 982);
    }
}
