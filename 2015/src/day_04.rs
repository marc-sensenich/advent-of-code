use advent_of_code::{read_file_to_string, read_lines};
use log::{debug, log_enabled, Level};
use md5;
use std::fmt::format;
use std::path::Path;

pub fn part_one(input_path: &Path) -> u64 {
    let input: String = match read_file_to_string(input_path) {
        Ok(_input) => _input,
        Err(_) => String::from(""),
    };

    solve(input.as_str(), 5)
}

pub fn part_two(input_path: &Path) -> u64 {
    let input: String = match read_file_to_string(input_path) {
        Ok(_input) => _input,
        Err(_) => String::from(""),
    };

    solve(input.as_str(), 6)
}

fn solve(input: &str, length: usize) -> u64 {
    let mut answer: u64 = 0;
    loop {
        let digest = md5::compute(format!("{}{}", input, answer).as_bytes());

        if format!("{:x}", digest).starts_with(&"0".repeat(length)) {
            break;
        }

        answer += 1;
    }
    answer
}
