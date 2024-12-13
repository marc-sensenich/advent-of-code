use advent_of_code::{read_file_to_string, read_lines};
use log::{debug, log_enabled, Level};
use regex::Regex;
use std::path::Path;

pub fn part_one(input_path: &Path) -> u32 {
    match read_file_to_string(input_path) {
        Ok(input) => solve_part_one(parse_input(input.as_str())),
        _ => u32::MAX,
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

type Prize = (u32, u32);
type Button = (u32, u32);

type Cost = (u32, u32);

#[derive(Debug)]
struct Machine {
    button_a: Button,
    button_b: Button,
    prize: Prize,
}

impl Machine {
    pub fn new(button_a: Button, button_b: Button, prize: Prize) -> Machine {
        Machine {
            button_a,
            button_b,
            prize,
        }
    }
}

type Machines = Vec<Machine>;

fn parse_input(input: &str) -> Machines {
    let regex = Regex::new(r"(?m)Button A: X\+(?<ax>\d+),\sY\+(?<ay>\d+)\nButton B: X\+(?<bx>\d+),\sY\+(?<by>\d+)\nPrize: X=(?<x_goal>\d+),\sY=(?<y_goal>\d+)").unwrap();

    regex
        .captures_iter(input)
        .map(|capture| {
            Machine::new(
                (
                    capture["ax"].parse::<u32>().unwrap(),
                    capture["ay"].parse::<u32>().unwrap(),
                ),
                (
                    capture["bx"].parse::<u32>().unwrap(),
                    capture["by"].parse::<u32>().unwrap(),
                ),
                (
                    capture["x_goal"].parse::<u32>().unwrap(),
                    capture["y_goal"].parse::<u32>().unwrap(),
                ),
            )
        })
        .collect::<Machines>()
}

fn solve_part_one(machines: Machines) -> u32 {
    machines
        .iter()
        .map(|m| determine_minimum_tokens_to_win_prize(m.prize, m.button_a, m.button_b))
        .filter(|c| c.is_some())
        .map(|c| c.unwrap().0 + c.unwrap().1)
        .sum()
}

fn determine_minimum_tokens_to_win_prize(
    prize: Prize,
    button_a: Button,
    button_b: Button,
) -> Option<Cost> {
    let mut cost: Option<Cost> = None;
    let mut total_cost: u32 = 0;
    let mut min_cost: u32 = u32::MAX;

    for (a_pushes, b_pushes) in possible_button_pushes(prize.0, button_a.0, button_b.0, 100) {
        let target: u32 = prize.1;
        let result: u32 = a_pushes * button_a.1 + b_pushes * button_b.1;

        if result == target {
            let local_cost: Cost = (a_pushes * 3, b_pushes);
            total_cost = local_cost.0 + local_cost.1;

            if total_cost < min_cost {
                cost = Some(local_cost);
                min_cost = total_cost;
            }
        }
    }

    cost
}

fn possible_button_pushes(
    target: u32,
    a_increment: u32,
    b_increment: u32,
    max_pushes: u32,
) -> Vec<(u32, u32)> {
    let mut result: u32 = 0;
    let mut a_result: u32 = 0;
    let mut b_result: u32 = 0;

    let mut possible_push_counts: Vec<(u32, u32)> = vec![];

    for i in 1..=max_pushes {
        a_result = a_increment * i;
        match a_result >= target {
            true => {}
            false => {
                for j in 1..=max_pushes {
                    b_result = b_increment * j;

                    match b_result > target {
                        true => {}
                        false => {
                            result = a_result + b_result;
                            if result == target {
                                possible_push_counts.push((i, j));
                            }
                        }
                    }
                }
            }
        }
    }

    possible_push_counts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn possible_button_pushes_with_results_first_example_x() {
        let expected: Vec<(u32, u32)> = vec![(69, 87), (80, 40)];
        let got: Vec<(u32, u32)> = possible_button_pushes(8400, 94, 22, 100);
        assert_eq!(got, expected);
    }

    #[test]
    fn possible_button_pushes_with_results_first_example_y() {
        let expected: Vec<(u32, u32)> = vec![(13, 74), (80, 40)];
        let got: Vec<(u32, u32)> = possible_button_pushes(5400, 34, 67, 100);
        assert_eq!(got, expected);
    }

    #[test]
    fn possible_button_pushes_without_results() {
        let expected: Vec<(u32, u32)> = vec![];
        let got: Vec<(u32, u32)> = possible_button_pushes(12176, 66, 21, 100);
        assert_eq!(got, expected);
    }

    #[test]
    fn determine_minimum_tokens_to_win_prize_first_example() {
        let expected: Option<Cost> = Some((240, 40));
        let got = determine_minimum_tokens_to_win_prize((8400, 5400), (94, 34), (22, 67));

        assert_eq!(got, expected);
    }

    #[test]
    fn solve_part_one_example() {
        let machines: Vec<Machine> = vec![
            Machine::new((94, 34), (22, 67), (8400, 5400)),
            Machine::new((26, 66), (67, 21), (12748, 12176)),
            Machine::new((17, 86), (84, 37), (7870, 6450)),
            Machine::new((69, 23), (27, 71), (18641, 10279)),
        ];

        let expected: u32 = 480;
        let got: u32 = solve_part_one(machines);

        assert_eq!(got, expected);
    }
}
