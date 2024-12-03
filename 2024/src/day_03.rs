use advent_of_code::read_file_to_string;
use log::{debug, log_enabled, Level};
use regex::Regex;
use std::path::Path;

pub fn part_one(input_path: &Path) -> i32 {
    let mut result: i32 = 0;

    if let Ok(input) = read_file_to_string(input_path) {
        let regex =
            Regex::new(r"(?m)mul\((?<left_operand>\d{1,3}),(?<right_operand>\d{1,3})\)").unwrap();

        result = regex
            .captures_iter(&input)
            .map(|capture| {
                capture["left_operand"].parse::<i32>().unwrap()
                    * capture["right_operand"].parse::<i32>().unwrap()
            })
            .sum::<i32>()
    }

    result
}

pub fn part_two(input_path: &Path) -> i32 {
    let mut instructions_enabled: bool = true;
    let mut instruction_results: Vec<i32> = vec![];

    if let Ok(input) = read_file_to_string(input_path) {
        let regex = Regex::new(r"(?m)mul\((?<left_operand>\d{1,3}),(?<right_operand>\d{1,3})\)|(?<do>do\(\))|(?<do_not>don't\(\))").unwrap();

        for capture in regex.captures_iter(&input) {
            if capture.name("do").is_some() {
                instructions_enabled = true;
                continue
            }

            if capture.name("do_not").is_some() {
                instructions_enabled = false;
                continue
            }

            if instructions_enabled {
                instruction_results.push(capture["left_operand"].parse::<i32>().unwrap()
                    * capture["right_operand"].parse::<i32>().unwrap());
            }
        }
    }

    instruction_results.iter().sum::<i32>()
}
