use advent_of_code::read_lines;
use log::{debug, log_enabled, Level};
use std::collections::VecDeque;
use std::path::Path;

pub fn part_one(input_path: &Path) -> u64 {
    let equations: Equations = parse_input(input_path);
    solve(
        &equations,
        &vec![Operation::Addition, Operation::Multiplication],
    )
}

pub fn part_two(input_path: &Path) -> u64 {
    let equations: Equations = parse_input(input_path);
    solve(
        &equations,
        &vec![
            Operation::Addition,
            Operation::Multiplication,
            Operation::Concatenation,
        ],
    )
}

fn solve(equations: &Equations, operations: &Vec<Operation>) -> u64 {
    equations
        .iter()
        .filter(|&e| is_operation_possible(e.operators.clone(), 0, e.solution, operations) != 0)
        .map(|e| e.solution)
        .sum::<u64>()
}

fn parse_input(input_path: &Path) -> Equations {
    let mut equations: Equations = Equations::new();

    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(result) = line {
                let split_result: Vec<&str> = result.split(": ").collect::<Vec<_>>();
                equations.push(Equation::new(
                    split_result[0].parse().unwrap(),
                    VecDeque::from(
                        split_result[1]
                            .split_whitespace()
                            .filter(|s| s.parse::<u64>().is_ok())
                            .map(|s| s.parse::<u64>().unwrap())
                            .collect::<Vec<_>>(),
                    ),
                ))
            }
        }
    }

    equations
}

fn is_operation_possible(
    mut operands: VecDeque<u64>,
    accumulator: u64,
    solution: u64,
    allowed_operations: &Vec<Operation>,
) -> u64 {
    let mut successful_operations: u64 = 0;

    if let Some(r) = operands.pop_front() {
        for operation in allowed_operations {
            match operation {
                Operation::Addition => {
                    let result: u64 = accumulator + r;

                    if operands.is_empty() {
                        if result == solution {
                            successful_operations += 1;
                        }
                    }
                    successful_operations += is_operation_possible(
                        operands.clone(),
                        result,
                        solution,
                        allowed_operations,
                    );
                }
                Operation::Multiplication => {
                    let result: u64 = accumulator * r;
                    if operands.is_empty() {
                        if result == solution {
                            successful_operations += 1;
                        }
                    }
                    successful_operations += is_operation_possible(
                        operands.clone(),
                        result,
                        solution,
                        allowed_operations,
                    );
                }
                Operation::Concatenation => {
                    let result: u64 = format!("{}{}", accumulator, r).parse::<u64>().unwrap();

                    if operands.is_empty() {
                        if result == solution {
                            successful_operations += 1;
                        }
                    }

                    successful_operations += is_operation_possible(
                        operands.clone(),
                        result,
                        solution,
                        allowed_operations,
                    );
                }
            }
        }
    }

    successful_operations
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_simple_addition() {
        assert_eq!(
            is_operation_possible(
                VecDeque::from(vec![10, 19]),
                0,
                190,
                &vec![Operation::Addition, Operation::Multiplication]
            ),
            1
        );
    }

    #[test]
    fn multiple_solutions() {
        assert_eq!(
            is_operation_possible(
                VecDeque::from(vec![81, 40, 27]),
                0,
                3267,
                &vec![Operation::Addition, Operation::Multiplication]
            ),
            2
        );
    }

    #[test]
    fn once_with_multiple_numbers() {
        assert_eq!(
            is_operation_possible(
                VecDeque::from(vec![11, 6, 16, 20]),
                0,
                292,
                &vec![Operation::Addition, Operation::Multiplication]
            ),
            1
        );
    }

    #[test]
    fn concatenation() {
        assert_eq!(
            is_operation_possible(
                VecDeque::from(vec![6, 8, 6, 15]),
                0,
                7290,
                &vec![
                    Operation::Addition,
                    Operation::Multiplication,
                    Operation::Concatenation,
                ]
            ),
            2
        );
    }
}

enum Operation {
    Addition,
    Multiplication,
    Concatenation,
}

#[derive(Debug)]
struct Equation {
    solution: u64,
    operators: VecDeque<u64>,
}

impl Equation {
    pub fn new(solution: u64, operators: VecDeque<u64>) -> Equation {
        Equation {
            solution,
            operators,
        }
    }
}

type Equations = Vec<Equation>;
