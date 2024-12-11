use advent_of_code::{read_file_to_string, read_lines};
use clap::builder::{Str, TypedValueParser};
use log::{debug, log_enabled, Level};
use std::path::Path;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn blink_with_zero() {
        assert_eq!(blink(&"0".to_string()), vec![String::from("1")]);
    }

    #[test]
    fn blink_even() {
        assert_eq!(
            blink(&"123456".to_string()),
            vec![String::from("123"), String::from("456")]
        )
    }

    #[test]
    fn blink_even_with_leading_zeroes() {
        assert_eq!(
            blink(&"1000".to_string()),
            vec![String::from("10"), String::from("0")]
        )
    }

    #[test]
    fn blink_default_rule() {
        assert_eq!(blink(&"999".to_string()), vec![String::from("2021976")]);
    }

    #[test]
    fn solve_part_one_example_short() {
        assert_eq!(
            solve_part_one(vec!["125".to_string(), "17".to_string()], 6),
            22
        )
    }

    #[test]
    fn solve_part_one_example_long() {
        assert_eq!(
            solve_part_one(vec!["125".to_string(), "17".to_string()], 25),
            55312
        )
    }
}

fn blink(value: &String) -> Vec<String> {
    // If the stone is engraved with the number 0,
    //   it is replaced by a stone engraved with the number 1.
    // If the stone is engraved with a number that has an even number of digits,
    //   it is replaced by two stones.
    //   The left half of the digits are engraved on the new left stone,
    //   and the right half of the digits are engraved on the new right stone.
    //   (The new numbers don't keep extra leading zeroes: 1000 would become stones 10 and 0.)
    // If none of the other rules apply, the stone is replaced by a new stone;
    //   the old stone's number multiplied by 2024 is engraved on the new stone.
    if value == "0" {
        vec![String::from('1')]
    } else if value.len() % 2 == 0 {
        let (left, right) = value.split_at(value.len() / 2);
        vec![
            left.parse::<u64>().unwrap().to_string(),
            right.parse::<u64>().unwrap().to_string(),
        ]
    } else {
        vec![(value.parse::<u64>().unwrap() * 2024).to_string()]
    }
}

fn solve_part_one(stones: Vec<String>, iterations: usize) -> u64 {
    let mut starting_stones: Vec<String> = stones.clone();
    let mut updated_stones: Vec<String> = vec![];
    debug!("Initial arrangement: \n{:?}", starting_stones);

    let mut insert_index: usize = 0;

    for iteration in 0..iterations {
        insert_index = 0;
        for i in 0..starting_stones.len() {
            let stone: String = starting_stones[i].clone();

            for (j, updated_stone) in blink(&stone).iter().enumerate() {
                updated_stones.insert(insert_index, updated_stone.clone());
                insert_index += 1;
            }
        }

        starting_stones = updated_stones.clone();
        updated_stones.clear();

        debug!("After {} blink \n{:?}", iteration + 1, starting_stones);
    }

    starting_stones.len() as u64
}

pub fn part_one(input_path: &Path) -> u64 {
    match read_file_to_string(input_path) {
        Ok(input) => solve_part_one(
            input
                .split_whitespace()
                .map(|s| String::from(s))
                .collect::<Vec<String>>(),
            25,
        ),
        _ => 0,
    }
}

pub fn part_two(input_path: &Path) -> u64 {
    0
}
