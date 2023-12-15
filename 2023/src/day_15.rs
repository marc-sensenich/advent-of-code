use std::io::BufRead;
use advent_of_code::{read_lines, read_file_to_string};
use log::{debug, log_enabled, Level};
use std::path::Path;

fn holiday_ascii_string_helper_algorithm(step: &str) -> u64 {
    let mut current_value: u64 = 0;
    for byte in step.as_bytes() {
        current_value = ((current_value + *byte as u64) * 17) % 256;
    }

    current_value
}

pub fn part_one(input_path: &Path) -> u64 {
    let mut input: String = String::from("");

    if let Ok(_input) = read_file_to_string(input_path) {
        input = _input;
    }

    input
        .split(",")
        .map(|m| holiday_ascii_string_helper_algorithm(m)).sum::<u64>()
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
