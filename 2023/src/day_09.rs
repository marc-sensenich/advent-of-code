use advent_of_code::read_lines;
use log::debug;
use std::path::Path;

fn determine_next_number_in_sequence(sequence: Vec<i64>) -> i64 {
    let next: Vec<i64> = sequence
        .windows(2)
        .map(|w| w[1] - w[0])
        .collect::<Vec<i64>>();
    debug!("{:?}", next);

    if next.iter().sum::<i64>() == 0 {
        return sequence.last().copied().unwrap();
    }

    sequence.last().copied().unwrap() + determine_next_number_in_sequence(next)
}

pub fn part_one(input_path: &Path) -> i64 {
    let mut sum: i64 = 0;
    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(result) = line {
                debug!("{}", result);
                sum += determine_next_number_in_sequence(
                    result
                        .split_whitespace()
                        .flat_map(|x| x.parse::<i64>())
                        .collect::<Vec<i64>>(),
                );
            }
        }
    }

    sum
}

pub fn part_two(input_path: &Path) -> i64 {
    let mut sum: i64 = 0;
    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(result) = line {
                debug!("{}", result);
                sum += determine_next_number_in_sequence(
                    result
                        .split_whitespace()
                        .rev()
                        .flat_map(|x| x.parse::<i64>())
                        .collect::<Vec<i64>>(),
                );
            }
        }
    }

    sum
}
