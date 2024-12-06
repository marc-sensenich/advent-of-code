use advent_of_code::read_lines;
use log::{debug, log_enabled, Level};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug)]
struct Rule {
    pub x: i32,
    pub y: i32,
}

impl Rule {
    pub fn new(x: i32, y: i32) -> Rule {
        Rule { x, y }
    }

    pub fn is_met(&self, update: &Vec<i32>) -> bool {
        match update.iter().position(|&x| x == self.x) {
            Some(x_index) => match update.iter().position(|&y| y == self.y) {
                Some(y_index) => y_index > x_index,
                None => true,
            },
            None => true,
        }
    }
}

type Rules = Vec<Rule>;
type Updates = Vec<Vec<i32>>;

fn parse_input(input_path: &Path) -> (Rules, Updates) {
    let mut rules: Rules = Rules::new();
    let mut updates: Updates = Updates::new();

    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(result) = line {
                if result.contains("|") {
                    let split_rule = result
                        .split("|")
                        .filter(|s| s.parse::<i32>().is_ok())
                        .map(|s| s.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>();
                    if split_rule.len() == 2 {
                        rules.push(Rule::new(split_rule[0], split_rule[1]));
                    }
                } else if result.contains(",") {
                    updates.push(result
                        .split(",")
                        .filter(|s| s.parse::<i32>().is_ok())
                        .map(|s| s.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>()
                    );
                }
            }
        }
    }

    (rules, updates)
}

fn solve_part_one(rules: Rules, mut updates: Updates) -> i32 {
    updates
        .iter_mut()
        .filter(|u| {
            rules
                .iter()
                .map(|r| r.is_met(&u))
                .reduce(|acc, e| acc && e)
                .unwrap()
        })
        .map(|u| {
            let midpoint: usize = (u.len() as f32 / 2.0).floor() as usize;
            u[midpoint]
        })
        .sum::<i32>()
}

pub fn part_one(input_path: &Path) -> i32 {
    let (rules, mut updates) = parse_input(input_path);

    solve_part_one(rules, updates)
}

pub fn part_two(input_path: &Path) -> i32 {
    0
}
