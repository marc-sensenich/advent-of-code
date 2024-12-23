use std::collections::{HashMap, HashSet};
use advent_of_code::{read_file_to_string, read_lines};
use log::{debug, log_enabled, Level};
use std::path::Path;

#[derive(Debug)]
struct Node {
    id: String,
    neighbors: HashSet<String>,
}

impl Node {
    fn new(id: String) -> Node {
        Node {
            id,
            neighbors: HashSet::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_part_one_example() {
        let input = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

        assert_eq!(solve_part_one(input), 7);
    }
}

fn parse_input(input: &str) -> HashMap<String, Node> {
    let mut result: HashMap<String, Node> = HashMap::new();

    input
        .lines()
        .map(|i| i.split("-"))
        .filter(|s| s.clone().count() == 2)
        .for_each(|mut s| {
            if let Some(l) = s.next() {
                if let Some(r) = s.next() {
                    let left = result.entry(l.to_string()).or_insert(Node::new(l.to_string()));
                    left.neighbors.insert(r.to_string());

                    let right = result.entry(r.to_string()).or_insert(Node::new(r.to_string()));
                    right.neighbors.insert(l.to_string());
                }
            }
        });

    result
}

fn solve_part_one(input: &str) -> u64 {
    let nodes = parse_input(input);
    let mut possible_connections: HashSet<String> = HashSet::new();

    nodes
        .iter()
        .filter(|(id, _)| id.starts_with('t'))
        .for_each(|(id, node)| {
            for neighbor_id in &node.neighbors {
                if let Some(neighbor) = nodes.get(neighbor_id) {
                    for subneighbor_id in &neighbor.neighbors {
                        if let Some(subneighbor) = nodes.get(subneighbor_id) {
                            if subneighbor.neighbors.contains(id) {
                                let mut connection: Vec<String> = vec![id.to_string(), neighbor_id.to_string(), subneighbor_id.to_string()];
                                connection.sort();
                                possible_connections.insert(connection.join(","));
                            }
                        }
                    }
                }
            }
        });

    possible_connections.len() as u64
}

pub fn part_one(input_path: &Path) -> u64 {
    match read_file_to_string(input_path) {
        Ok(input) => solve_part_one(input.as_str()),
        _ => 0
    }
}

pub fn part_two(input_path: &Path) -> i32 {
    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(result) = line {
                debug!("{}", result);
            }
        }
    }

    0
}
