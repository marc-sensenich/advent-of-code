use advent_of_code::read_lines;
use log::{debug, log_enabled, Level};
use std::iter::zip;
use std::path::Path;

pub fn part_one(input_path: &Path) -> i32 {
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(result) = line {
                let mut split_result = result.split_whitespace();
                left.push(split_result.next().unwrap().parse().unwrap());
                right.push(split_result.next().unwrap().parse().unwrap());
            }
        }
    }

    left.sort();
    right.sort();

    zip(left, right).map(|(x, y)| (x - y).abs()).sum()
}

pub fn part_two(input_path: &Path) -> i32 {
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(result) = line {
                let mut split_result = result.split_whitespace();
                left.push(split_result.next().unwrap().parse().unwrap());
                right.push(split_result.next().unwrap().parse().unwrap());
            }
        }
    }

    left.iter()
        .map(|x| x * right.iter().filter(|y| *y == x).count() as i32)
        .collect::<Vec<i32>>()
        .iter()
        .sum()
}
