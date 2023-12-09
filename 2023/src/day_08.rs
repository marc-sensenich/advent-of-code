use advent_of_code::read_lines;
use num::integer::lcm;
use regex::Regex;
use std::collections::HashMap;
use std::path::Path;

pub fn part_one(input_path: &Path) -> i64 {
    let node_regex = Regex::new(r"[A-Z]{3}").unwrap();
    let mut instructions: Vec<char> = vec![];
    let mut nodes: HashMap<String, (String, String)> = HashMap::new();
    let mut current_node_id: String = String::from("AAA");

    if let Ok(lines) = read_lines(input_path) {
        for (idx, line) in lines.enumerate() {
            if let Ok(result) = line {
                match idx {
                    0 => instructions = result.chars().collect::<Vec<char>>(),
                    _ => {
                        if result.is_empty() {
                            continue;
                        }

                        let mut node_captures = node_regex.captures_iter(&result);
                        let (id, []) = node_captures.nth(0).unwrap().extract();
                        let (left, []) = node_captures.nth(0).unwrap().extract();
                        let (right, []) = node_captures.nth(0).unwrap().extract();

                        nodes.insert(id.to_string(), (left.to_string(), right.to_string()));
                    }
                }
            }
        }
    }

    let instructions_count: usize = instructions.len();
    let mut current_index: usize = 0;
    let mut left_or_right: char = 'z';
    let mut current_node: (String, String) = nodes.get(&current_node_id).unwrap().clone();
    let mut steps: i64 = 0;

    while current_node_id != "ZZZ" {
        left_or_right = instructions[current_index];
        current_node_id = match left_or_right {
            'L' => current_node.0.clone(),
            'R' => current_node.1.clone(),
            _ => todo!(),
        };
        current_index = (current_index + 1) % instructions_count;
        current_node = nodes.get(&current_node_id).unwrap().clone();
        steps += 1;
    }

    steps
}

pub fn part_two(input_path: &Path) -> i64 {
    let node_regex = Regex::new(r"[A-Z\d]{3}").unwrap();
    let mut instructions: Vec<char> = vec![];
    let mut nodes: HashMap<String, (String, String)> = HashMap::new();
    let _current_node_id: String = String::from("AAA");
    let mut starting_nodes: Vec<String> = vec![];

    if let Ok(lines) = read_lines(input_path) {
        for (idx, line) in lines.enumerate() {
            if let Ok(result) = line {
                match idx {
                    0 => instructions = result.chars().collect::<Vec<char>>(),
                    _ => {
                        if result.is_empty() {
                            continue;
                        }

                        let mut node_captures = node_regex.captures_iter(&result);
                        let (id, []) = node_captures.nth(0).unwrap().extract();
                        let (left, []) = node_captures.nth(0).unwrap().extract();
                        let (right, []) = node_captures.nth(0).unwrap().extract();

                        if id.ends_with('A') {
                            starting_nodes.push(String::from(id));
                        }

                        nodes.insert(id.to_string(), (left.to_string(), right.to_string()));
                    }
                }
            }
        }
    }

    let instructions_count: usize = instructions.len();
    let mut current_index: usize = 0;
    let mut left_or_right: char = 'z';
    let mut current_node: (String, String) = (String::from(""), String::from(""));
    let mut steps: i64 = 0;
    let mut next_node_id: String = String::from("");

    let mut distances: Vec<i64> = vec![];

    for current_node_id in starting_nodes {
        next_node_id = current_node_id.to_string();
        current_index = 0;
        steps = 0;
        while !next_node_id.ends_with('Z') {
            left_or_right = instructions[current_index];

            steps += 1;
            current_node = nodes.get(&next_node_id).unwrap().clone();

            next_node_id = match left_or_right {
                'L' => current_node.0.clone(),
                'R' => current_node.1.clone(),
                _ => todo!(),
            };

            current_index = (current_index + 1) % instructions_count;
        }

        distances.push(steps);
    }

    distances.iter().copied().reduce(|a, b| lcm(a, b)).unwrap()
}
