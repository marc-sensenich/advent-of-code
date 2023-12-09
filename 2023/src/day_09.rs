use advent_of_code::read_lines;
use log::debug;
use std::path::Path;

pub fn part_one(input_path: &Path) -> i64 {
    read_lines(input_path)
        .unwrap()
        .map(|l| {
            determine_next_number_in_sequence(
                l.unwrap()
                    .split_whitespace()
                    .flat_map(|x| x.parse::<i64>())
                    .collect::<Vec<i64>>(),
            )
        })
        .sum::<i64>()
}

pub fn part_two(input_path: &Path) -> i64 {
    read_lines(input_path)
        .unwrap()
        .map(|l| {
            determine_next_number_in_sequence(
                l.unwrap()
                    .split_whitespace()
                    .rev()
                    .flat_map(|x| x.parse::<i64>())
                    .collect::<Vec<i64>>(),
            )
        })
        .sum::<i64>()
}

fn determine_next_number_in_sequence(sequence: Vec<i64>) -> i64 {
    let next: Vec<i64> = sequence
        .windows(2)
        .map(|w| w[1] - w[0])
        .collect::<Vec<i64>>();

    match next.iter().sum::<i64>() {
        0 => sequence.last().copied().unwrap(),
        _ => sequence.last().copied().unwrap() + determine_next_number_in_sequence(next),
    }
}
