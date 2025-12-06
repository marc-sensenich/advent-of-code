use advent_of_code::{read_file_to_string};
use std::collections::VecDeque;
use std::path::Path;

fn solve(input: Vec<Problem>) -> u64 {
    input.iter().map(|x| x.fold()).sum()
}

pub fn part_one(input_path: &Path) -> u64 {
    match read_file_to_string(input_path) {
        Ok(input) => solve(parse_input_part_one(input)),
        Err(_) => 0,
    }
}

pub fn part_two(input_path: &Path) -> u64 {
    match read_file_to_string(input_path) {
        Ok(input) => solve(parse_input_part_two(input)),
        Err(_) => 0,
    }
}

fn parse_input_part_one(input: String) -> Vec<Problem> {
    let mut result: Vec<Problem> = vec![];
    let mut input_as_single_vec: Vec<_> = vec![];
    let line_count_zero_indexed: usize = input.lines().count() - 1;

    input
        .lines()
        .for_each(|x| input_as_single_vec.append(&mut x.split_whitespace().collect::<Vec<_>>()));
    let line_length_zero_indexed: usize = input_as_single_vec
        .iter()
        .filter(|x| !x.parse::<u64>().is_ok())
        .count();

    let mut operands: Vec<u64> = vec![];
    let mut operator: String = String::new();

    for i in 0..line_length_zero_indexed {
        operands.clear();
        for j in 0..=line_count_zero_indexed {
            let idx = i + (j * (line_length_zero_indexed));
            let val = input_as_single_vec[idx];

            if j == line_count_zero_indexed {
                operator = val.to_string();
            } else {
                operands.push(val.parse().unwrap());
            }
        }

        result.push(Problem::new(operands.clone(), operator.clone()));
    }

    result
}
fn parse_input_part_two(input: String) -> Vec<Problem> {
    let mut result: Vec<Problem> = vec![];
    let lines: Vec<&str> = input.lines().collect();
    let mut operators: VecDeque<String> = lines
        .last()
        .unwrap()
        .split_whitespace()
        .map(|x| x.to_string())
        .collect();
    let char_count: usize = lines.first().unwrap().len();

    let mut operands: Vec<u64> = vec![];
    let mut operand: Vec<char> = vec![];

    for j in 0..char_count {
        for i in 0..(lines.len() - 1) {
            let chars: Vec<char> = lines[i].chars().collect();

            if chars[j].is_numeric() {
                operand.push(chars[j]);
            }
        }

        if !operand.is_empty() {
            operands.push(operand.iter().collect::<String>().parse::<u64>().unwrap())
        } else {
            result.push(Problem::new(
                operands.clone(),
                operators.pop_front().unwrap(),
            ));
            operands.clear();
        }
        operand.clear();
    }

    if !operands.is_empty() {
        result.push(Problem::new(
            operands.clone(),
            operators.pop_front().unwrap(),
        ));
    }

    result
}

#[derive(PartialEq, Debug)]
struct Problem {
    operands: Vec<u64>,
    operator: String,
}

impl Problem {
    pub fn new(operands: Vec<u64>, operator: String) -> Problem {
        Problem { operands, operator }
    }

    pub fn fold(&self) -> u64 {
        match self.operator.as_str() {
            "*" => self.operands.iter().fold(1, |acc, x| acc * x),
            "+" => self.operands.iter().fold(0, |acc, x| acc + x),
            _ => 0,
        }
    }
}

#[cfg(test)]
mod test_part_one {
    use super::*;

    fn expected_input() -> Vec<Problem> {
        vec![
            Problem::new(vec![123, 45, 6], "*".to_string()),
            Problem::new(vec![328, 64, 98], "+".to_string()),
            Problem::new(vec![51, 387, 215], "*".to_string()),
            Problem::new(vec![64, 23, 314], "+".to_string()),
        ]
    }

    #[test]
    fn test_parse_input() {
        let result: Vec<Problem> = parse_input_part_one(
            "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ".to_string(),
        );

        assert_eq!(expected_input(), result);
    }

    #[test]
    fn test_solve_part_one_example() {
        assert_eq!(solve(expected_input()), 4277556);
    }
}

#[cfg(test)]
mod test_part_two {
    use super::*;
    fn expected_input() -> Vec<Problem> {
        vec![
            Problem::new(vec![1, 24, 356], "*".to_string()),
            Problem::new(vec![369, 248, 8], "+".to_string()),
            Problem::new(vec![32, 581, 175], "*".to_string()),
            Problem::new(vec![623, 431, 4], "+".to_string()),
        ]
    }

    #[test]
    fn test_parse_input() {
        let result: Vec<Problem> = parse_input_part_two(
            "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ".to_string(),
        );
        assert_eq!(expected_input(), result);
    }

    #[test]
    fn test_solve_example() {
        assert_eq!(solve(expected_input()), 3263827);
    }
}
