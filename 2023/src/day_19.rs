use advent_of_code::{read_file_to_string, read_lines};
use log::{debug, log_enabled, Level};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::collections::HashMap;
use std::fmt;
use std::path::Path;

pub fn part_one(input_path: &Path) -> u64 {
    let mut rules: Rules = Rules::new();
    let mut parts: Parts = Parts::new();

    if let Ok(input) = read_file_to_string(input_path) {
        (rules, parts) = parse_input(input);
    }

    parts
        .iter()
        .map(|p| {
            if (rules.get("in").unwrap()).run(&p, &rules) {
                return p.total_rating();
            }

            0
        })
        .sum()
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

#[derive(Debug)]
enum Comparison {
    GreaterThan,
    LessThan,
    Default,
}

#[derive(Debug)]
struct Operation {
    left_side_key: String,
    comparison: Comparison,
    right_side: u64,
    result: String,
}

impl Operation {
    pub fn new(
        left_side_key: String,
        comparison: Comparison,
        right_side: u64,
        result: String,
    ) -> Operation {
        Operation {
            left_side_key,
            comparison,
            right_side,
            result,
        }
    }

    pub fn run(&self, left_side: u64) -> Option<String> {
        let result = match &self.comparison {
            Comparison::GreaterThan => left_side > self.right_side,
            Comparison::LessThan => left_side < self.right_side,
            Comparison::Default => true,
        };

        match result {
            true => return Some(self.result.clone()),
            false => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl Part {
    pub fn new(x: u64, m: u64, a: u64, s: u64) -> Part {
        Part { x, m, a, s }
    }

    pub fn get(&self, key: &str) -> u64 {
        match key {
            "x" => self.x,
            "m" => self.m,
            "a" => self.a,
            "s" => self.s,
            _ => 0,
        }
    }

    pub fn total_rating(&self) -> u64 {
        self.x + self.m + self.a + self.s
    }
}

type Rules = HashMap<String, Rule>;
type Parts = Vec<Part>;

#[derive(Debug)]
struct Rule {
    id: String,
    operations: Vec<Operation>,
}

impl Rule {
    pub fn new(id: String, operations: Vec<Operation>) -> Rule {
        Rule { id, operations }
    }

    pub fn run(&self, part: &Part, rules: &Rules) -> bool {
        for operation in &self.operations {
            if let Some(result) = operation.run(part.get(&operation.left_side_key)) {
                return match result.as_str() {
                    "A" => true,
                    "R" => false,
                    rule_id => (rules.get(&rule_id.to_string()).unwrap()).run(part, rules),
                };
            }
        }

        return false;
    }
}

fn parse_input(input: String) -> (Rules, Parts) {
    let mut rules: Rules = Rules::new();
    let mut parts: Parts = Parts::new();

    let split_input = input.split("\n\n").collect::<Vec<_>>();
    let rules_input = split_input.get(0).unwrap();
    let parts_input = split_input.get(1).unwrap();

    for rule_str in rules_input.lines() {
        let rule_id: &str = rule_str.split("{").collect::<Vec<_>>().get(0).unwrap();
        let rules_str: String = rule_str
            .split("{")
            .collect::<Vec<_>>()
            .get(1)
            .unwrap()
            .replace("}", "");
        let mut operations: Vec<Operation> = vec![];

        for r in rules_str.split(",").collect::<Vec<_>>() {
            if r.contains(":") {
                let mut comparison: Comparison = Comparison::GreaterThan;
                if r.contains("<") {
                    comparison = Comparison::LessThan;
                }
                let result: String = r.split(":").collect::<Vec<_>>().get(1).unwrap().to_string();
                let right_side: u64 = r
                    .chars()
                    .filter(|c| c.is_digit(10))
                    .collect::<String>()
                    .parse::<u64>()
                    .unwrap();
                let category: String = r.to_string().get(0..=0).unwrap().to_string();

                operations.push(Operation::new(category, comparison, right_side, result));
            } else {
                operations.push(Operation::new(
                    String::from(""),
                    Comparison::Default,
                    0,
                    r.to_string(),
                ));
            }
        }

        rules.insert(
            String::from(rule_id),
            Rule::new(String::from(rule_id), operations),
        );
    }

    for part_input in parts_input.lines() {
        let mut json_input = String::from(part_input);
        json_input = json_input.replace("=", ":");
        json_input = json_input.replace("x", "\"x\"");
        json_input = json_input.replace("m", "\"m\"");
        json_input = json_input.replace("a", "\"a\"");
        json_input = json_input.replace("s", "\"s\"");

        if let Ok(part) = serde_json::from_str(&json_input) {
            parts.push(part);
        }
    }

    (rules, parts)
}
