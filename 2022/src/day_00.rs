use advent_of_code::read_lines;
use std::path::Path;

pub fn part_one(input_path: &Path) -> i32 {
    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(result) = line {
                println!("{}", result);
            }
        }
    }

    0
}

pub fn part_two(input_path: &Path) -> i32 {
    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(result) = line {
                println!("{}", result);
            }
        }
    }

    0
}
