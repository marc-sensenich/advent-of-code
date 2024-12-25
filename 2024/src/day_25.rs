use advent_of_code::{read_file_to_string, read_lines};
use log::{debug, log_enabled, Level};
use std::collections::HashMap;
use std::iter::zip;
use std::path::Path;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_fits_lock_first_example() {
        let expected = false;
        let got = key_fits_lock(vec![0, 5, 3, 4, 3], vec![5, 0, 2, 1, 3]);

        assert_eq!(got, expected);
    }

    #[test]
    fn key_fits_lock_second_example() {
        let expected = false;
        let got = key_fits_lock(vec![0, 5, 3, 4, 3], vec![4, 3, 4, 0, 2]);

        assert_eq!(got, expected);
    }

    #[test]
    fn key_fits_lock_third_example() {
        let expected = true;
        let got = key_fits_lock(vec![0, 5, 3, 4, 3], vec![3, 0, 2, 0, 1]);

        assert_eq!(got, expected);
    }

    #[test]
    fn key_fits_lock_fourth_example() {
        let expected = false;
        let got = key_fits_lock(vec![1, 2, 0, 5, 3], vec![5, 0, 2, 1, 3]);

        assert_eq!(got, expected);
    }

    #[test]
    fn key_fits_lock_fifth_example() {
        let expected = true;
        let got = key_fits_lock(vec![1, 2, 0, 5, 3], vec![4, 3, 4, 0, 2]);

        assert_eq!(got, expected);
    }

    #[test]
    fn key_fits_lock_sixth_example() {
        let expected = true;
        let got = key_fits_lock(vec![1, 2, 0, 5, 3], vec![3, 0, 2, 0, 1]);

        assert_eq!(got, expected);
    }

    #[test]
    fn parse_input_example() {
        let input = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";

        let expected_keys = vec![
            vec![5, 0, 2, 1, 3],
            vec![4, 3, 4, 0, 2],
            vec![3, 0, 2, 0, 1],
        ];

        let expected_locks = vec![vec![0, 5, 3, 4, 3], vec![1, 2, 0, 5, 3]];

        let (got_keys, got_locks) = parse_input(input);

        assert_eq!(got_keys, expected_keys);
        assert_eq!(got_locks, expected_locks);
    }

    #[test]
    fn solve_part_one_example() {
        let input = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";

        let expected: u64 = 3;
        let got: u64 = solve_part_one(input);

        assert_eq!(got, expected);
    }
}

fn key_fits_lock(lock: Vec<i32>, key: Vec<i32>) -> bool {
    zip(lock, key)
        .map(|(l, k)| !key_code_overlaps_pin(l, k))
        .reduce(|acc, e| acc && e)
        .unwrap_or_default()
}

fn key_code_overlaps_pin(pin_height: i32, key_code: i32) -> bool {
    key_code >= (pin_height - 6).abs()
}

fn parse_input(input: &str) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let locks_and_keys = input.split("\n\n").collect::<Vec<_>>();
    let mut locks: Vec<Vec<i32>> = vec![];
    let mut keys: Vec<Vec<i32>> = vec![];

    let mut is_lock = false;
    let mut working_item: HashMap<usize, i32> = HashMap::new();

    locks_and_keys.iter().for_each(|l| {
        let lines = l.lines();
        let lines_length = lines.clone().count();
        lines.enumerate().for_each(|(idx, line)| {
            if idx == 0 {
                is_lock = line == "#####";
            } else if idx == lines_length - 1 {
                // continue;
            } else {
                line.chars().enumerate().for_each(|(cidx, c)| {
                    if c == '#' {
                        working_item
                            .entry(cidx)
                            .and_modify(|counter| *counter += 1)
                            .or_insert(1);
                    } else {
                        working_item.entry(cidx).or_insert(0);
                    }
                });
            }
        });

        if is_lock {
            locks.push(vec![
                *working_item.get(&0).unwrap(),
                *working_item.get(&1).unwrap(),
                *working_item.get(&2).unwrap(),
                *working_item.get(&3).unwrap(),
                *working_item.get(&4).unwrap(),
            ])
        } else {
            keys.push(vec![
                *working_item.get(&0).unwrap(),
                *working_item.get(&1).unwrap(),
                *working_item.get(&2).unwrap(),
                *working_item.get(&3).unwrap(),
                *working_item.get(&4).unwrap(),
            ])
        }

        working_item.clear();
    });

    (keys, locks)
}

fn solve_part_one(input: &str) -> u64 {
    let (keys, locks) = parse_input(input);
    let mut result: u64 = 0;

    keys.iter().for_each(|k| {
        locks.iter().for_each(|l| {
            if key_fits_lock(l.clone(), k.clone()) {
                result += 1;
            }
        })
    });

    result
}

pub fn part_one(input_path: &Path) -> u64 {
    match read_file_to_string(input_path) {
        Ok(input) => solve_part_one(input.as_str()),
        _ => 0,
    }
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
