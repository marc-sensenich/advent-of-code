use advent_of_code::{read_file_to_string, read_lines};
use log::{debug, log_enabled, Level};
use std::path::Path;
use std::process::id;

pub fn part_one(input_path: &Path) -> i32 {
    match read_file_to_string(input_path) {
        Ok(result) => result
            .chars()
            .map(|m| match m {
                '(' => 1,
                ')' => -1,
                _ => 0,
            })
            .sum(),
        Err(_) => i32::MIN,
    }
}

pub fn part_two(input_path: &Path) -> i32 {
    let mut floor: i32 = 0;

    match read_file_to_string(input_path) {
        Ok(result) => {
            for (idx, step) in result.chars().map(|m| match m {
                '(' => 1,
                ')' => -1,
                _ => 0,
            }).enumerate() {
                floor += step;
                if floor == -1 {
                    return (idx + 1) as i32
                }
            }

            i32::MIN
        },
        Err(_) => i32::MIN,
    }
}
