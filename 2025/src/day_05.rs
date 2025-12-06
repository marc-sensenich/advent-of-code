use advent_of_code::{read_file_to_string};
use std::collections::HashSet;
use std::ops::{RangeInclusive};
use std::path::Path;

#[cfg(test)]
mod tests {
    use super::*;

    fn expected_parsed_input() -> Option<(FreshIngredients, AvailableIngredients)> {
        let expected_available_ingredients: AvailableIngredients = vec![1, 5, 8, 11, 17, 32];
        let expected_fresh_ingredients: FreshIngredients =
            FreshIngredients::from_iter(vec![3..=5, 10..=14, 16..=20, 12..=18]);

        Some((expected_fresh_ingredients, expected_available_ingredients))
    }

    #[test]
    fn test_parse_input() {
        let input: String = "3-5
10-14
16-20
12-18

1
5
8
11
17
32"
        .to_string();

        assert_eq!(parse_input(input), expected_parsed_input())
    }

    #[test]
    fn test_solve_part_one_example() {
        let (fresh, available) = expected_parsed_input().unwrap();
        assert_eq!(solve_part_one(fresh, available), 3)
    }

    #[test]
    fn test_solve_part_two_example() {
        let (fresh, available) = expected_parsed_input().unwrap();
        assert_eq!(solve_part_two(fresh, available), 14)
    }
}

type FreshIngredients = HashSet<RangeInclusive<u64>>;
type AvailableIngredients = Vec<u64>;

fn parse_input(input: String) -> Option<(FreshIngredients, AvailableIngredients)> {
    let split_input: Vec<&str> = input.split("\n\n").collect::<Vec<&str>>();

    let available_ingredients: AvailableIngredients = split_input[1]
        .lines()
        .filter_map(|l| l.parse::<u64>().ok())
        .collect::<AvailableIngredients>();

    let mut fresh_ingredients: FreshIngredients = FreshIngredients::new();

    split_input[0].lines().for_each(|l| {
        let range = l
            .split("-")
            .filter_map(|s| s.parse::<u64>().ok())
            .collect::<Vec<u64>>();
        fresh_ingredients.insert(range[0]..=range[1]);
    });

    Some((fresh_ingredients, available_ingredients))
}

fn solve_part_one(fresh: FreshIngredients, available: AvailableIngredients) -> usize {
    let mut result: usize = 0;
    for i in available {
        for j in fresh.clone() {
            if j.contains(&i) {
                result += 1;
                break;
            }
        }
    }

    result
}

fn solve_part_two(_fresh: FreshIngredients, _available: AvailableIngredients) -> usize {
    // Too slow of a solution
    // let mut acc: HashSet<u64> = HashSet::new();
    // for j in fresh {
    //     acc = acc.union(&HashSet::from_iter(j)).map(|x| *x).collect();
    // }
    //
    // acc.len()
    0
}

pub fn part_one(input_path: &Path) -> usize {
    match read_file_to_string(input_path) {
        Ok(input) => match parse_input(input) {
            Some((fresh, available)) => solve_part_one(fresh, available),
            None => 0,
        },
        Err(_) => 0,
    }
}

pub fn part_two(input_path: &Path) -> usize {
    match read_file_to_string(input_path) {
        Ok(input) => match parse_input(input) {
            Some((fresh, available)) => solve_part_two(fresh, available),
            None => 0,
        },
        Err(_) => 0,
    }
}
