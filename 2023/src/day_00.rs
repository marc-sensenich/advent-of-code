use advent_of_code::read_lines;
use log::{debug, log_enabled, Level};
use std::path::Path;

pub fn part_one(input_path: &Path) -> u64 {
    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(result) = line {
                debug!("{}", result);
            }
        }
    }

    0
}

pub fn part_two(input_path: &Path) -> u64 {
    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(result) = line {
                debug!("{}", result);
            }
        }
    }

    0
}
