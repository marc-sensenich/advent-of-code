use advent_of_code::{read_file_to_string, read_lines};
use log::{debug, log_enabled, Level};
use std::path::Path;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mix_secret_number() {
        let expected: u64 = 37;
        let got: u64 = mix_secret_number(42, 15);

        assert_eq!(got, expected);
    }

    #[test]
    fn test_prune_secret_number() {
        let expected: u64 = 16113920;
        let got: u64 = prune_secret_number(100000000);

        assert_eq!(got, expected);
    }

    #[test]
    fn test_123_once() {
        let expected: u64 = 15887950;
        let got: u64 = calculate_secret_number(123, 1).0;

        assert_eq!(got, expected);
    }

    #[test]
    fn test_123_ten_times() {
        let expected: u64 = 5908254;
        let got: u64 = calculate_secret_number(123, 10).0;

        assert_eq!(got, expected);
    }

    #[test]
    fn test_parse_input() {
        let input: &str = "1
10
100
2024";
        let expected: Vec<u64> = vec![1, 10, 100, 2024];
        let got = parse_input(input);

        assert_eq!(got, expected);
    }

    #[test]
    fn test_solve_part_one() {
        let input: &str = "1
10
100
2024";
        let expected: u64 = 37327623;
        let got: u64 = solve_part_one(input, 2000);

        assert_eq!(got, expected);
    }
}

fn prune_secret_number(secret_number: u64) -> u64 {
    secret_number.rem_euclid(16777216)
}

fn mix_secret_number(secret_number: u64, mix_value: u64) -> u64 {
    secret_number ^ mix_value
}

fn calculate_secret_number(current: u64, cycles: usize) -> (u64, Vec<i32>) {
    let mut result: u64 = current;
    let mut result_len: usize = current.to_string().len();

    let mut price: i32 = result.to_string()[result_len - 1..result_len]
        .parse::<i32>()
        .unwrap();
    let mut prices: Vec<i32> = Vec::with_capacity(cycles + 1);
    prices.push(price);

    for _ in 0..cycles {
        result = prune_secret_number(mix_secret_number(result, result * 64));
        result = prune_secret_number(mix_secret_number(result, result / 32));
        result = prune_secret_number(mix_secret_number(result, result * 2048));

        result_len = result.to_string().len();
        price = result.to_string()[result_len - 1..result_len]
            .parse::<i32>()
            .unwrap();
        prices.push(price);
    }

    (result, prices)
}

fn parse_input(input: &str) -> Vec<u64> {
    input
        .lines()
        .filter(|i| i.parse::<u64>().is_ok())
        .map(|i| i.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}

fn solve_part_one(input: &str, cycles: usize) -> u64 {
    parse_input(input)
        .iter()
        .map(|i| calculate_secret_number(*i, cycles).0)
        .sum()
}

pub fn part_one(input_path: &Path) -> u64 {
    match read_file_to_string(input_path) {
        Ok(input) => solve_part_one(input.as_str(), 2000),
        _ => 0,
    }
}

pub fn part_two(input_path: &Path) -> u64 {
    0
}
