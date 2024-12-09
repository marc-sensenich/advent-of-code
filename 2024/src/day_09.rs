use advent_of_code::{read_file_to_string, read_lines};
use log::{debug, log_enabled, Level};
use std::path::Path;

pub fn part_one(input_path: &Path) -> u64 {
    match read_file_to_string(input_path) {
        Ok(input) => solve_part_one(parse_input(input.as_str())),
        _ => 0,
    }
}

fn solve_part_one(mut filesystem: Vec<String>) -> u64 {
    let mut head: usize = 0;
    let mut tail: usize = filesystem.len() - 1;

    while head < tail {
        let head_str: String = filesystem[head].clone();
        if head_str == "." {
            let mut tail_str: String = filesystem[tail].clone();
            match tail_str == "." {
                true => {
                    tail -= 1;
                    continue;
                }
                false => {
                    filesystem.swap(head, tail);
                    tail -= 1;
                }
            }
        }
        head += 1;
    }

    filesystem
        .iter()
        .enumerate()
        .map(|(idx, x)| {
            if *x != "." {
                if let Some(file_id) = x.parse::<u64>().ok() {
                    return file_id * idx as u64;
                }
            }

            0
        })
        .sum::<u64>()
}

#[cfg(test)]
mod parse_input_tests {
    use super::parse_input;

    #[test]
    fn simple() {
        assert_eq!(
            parse_input(&"12345"),
            vec!["0", ".", ".", "1", "1", "1", ".", ".", ".", ".", "2", "2", "2", "2", "2",]
        )
    }

    #[test]
    fn first_example() {
        assert_eq!(
            parse_input(&"2333133121414131402"),
            vec![
                "0", "0", ".", ".", ".", "1", "1", "1", ".", ".", ".", "2", ".", ".", ".", "3",
                "3", "3", ".", "4", "4", ".", "5", "5", "5", "5", ".", "6", "6", "6", "6", ".",
                "7", "7", "7", ".", "8", "8", "8", "8", "9", "9"
            ]
        )
    }
}

fn parse_input(input: &str) -> Vec<String> {
    let mut file_id: u64 = 0;
    let mut result: Vec<String> = vec![];

    for (idx, c) in input.chars().enumerate() {
        // If index is even, it's a file add n chars to the vector
        if let Some(file_block_size) = c.to_digit(10) {
            if idx % 2 == 0 {
                result.append(
                    &mut (0..file_block_size)
                        .map(|_| file_id.to_string())
                        .collect::<Vec<String>>(),
                );
                file_id += 1;
            } else {
                result.append(
                    &mut (0..file_block_size)
                        .map(|_| ".".to_string())
                        .collect::<Vec<String>>(),
                );
            }
        }
    }

    result
}

#[cfg(test)]
mod solve_part_one_tests {
    use super::{parse_input, solve_part_one};

    #[test]
    fn simple() {
        assert_eq!(solve_part_one(parse_input(&"12345")), 60)
    }

    #[test]
    fn first_example() {
        assert_eq!(solve_part_one(parse_input(&"2333133121414131402")), 1928)
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
