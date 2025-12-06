use advent_of_code::read_file_to_string;
use std::collections::VecDeque;
use std::path::Path;

pub fn part_one(input_path: &Path) -> u64 {
    let mut result: u64 = 0;
    if let Ok(input) = read_file_to_string(input_path) {
        result = solve(input, 2);
    }
    result
}

pub fn part_two(input_path: &Path) -> u64 {
    let mut result: u64 = 0;
    if let Ok(input) = read_file_to_string(input_path) {
        result = solve(input, 12);
    }
    result
}

fn solve(input: String, result_length: usize) -> u64 {
    input
        .lines()
        .map(|i| find_largest_joltage(i.to_string(), result_length))
        .sum()
}

fn find_largest_joltage(bank: String, result_length: usize) -> u64 {
    let mut result_vec: VecDeque<u64> = VecDeque::new();
    let mut bank_vec = bank
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|x| x as u64)
        .collect::<VecDeque<u64>>();
    let mut max: u64;
    let mut max_idx: usize = 0;

    while result_vec.len() != result_length {
        let mut val: u64;
        max = 0;

        for i in max_idx..=bank_vec.len() - (result_length - result_vec.len()) {
            val = bank_vec[i];

            if val > max {
                max = val;
                max_idx = i;
            }
        }

        result_vec.push_back(max);
        max_idx += 1;
        let rem: usize = bank_vec.len() - max_idx;
        if result_vec.len() + rem == result_length {
            result_vec.append(&mut bank_vec.split_off(max_idx));
        }
    }

    result_vec
        .iter()
        .map(|x| x.to_string())
        .collect::<String>()
        .parse::<u64>()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const RESULT_LENGTH: usize = 2;
    #[test]
    fn test_find_largest_joltage_one() {
        assert_eq!(
            find_largest_joltage("987654321111111".to_string(), RESULT_LENGTH),
            98,
        )
    }

    #[test]
    fn test_find_largest_joltage_two() {
        assert_eq!(
            find_largest_joltage("987654321111111".to_string(), RESULT_LENGTH),
            98,
        )
    }

    #[test]
    fn test_find_largest_joltage_three() {
        assert_eq!(
            find_largest_joltage("234234234234278".to_string(), RESULT_LENGTH),
            78,
        )
    }

    #[test]
    fn test_find_largest_joltage_four() {
        assert_eq!(
            find_largest_joltage("818181911112111".to_string(), RESULT_LENGTH),
            92,
        )
    }

    #[test]
    fn test_solve_part_one_example() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";

        assert_eq!(solve(input.to_string(), RESULT_LENGTH), 357,)
    }
}

#[cfg(test)]
mod tests_part_two {
    use super::*;

    const RESULT_LENGTH: usize = 12;

    #[test]
    fn test_find_largest_joltage_part_two_example_one() {
        assert_eq!(
            find_largest_joltage("987654321111111".to_string(), RESULT_LENGTH),
            987654321111,
        )
    }

    #[test]
    fn test_find_largest_joltage_part_two_example_two() {
        assert_eq!(
            find_largest_joltage("811111111111119".to_string(), RESULT_LENGTH),
            811111111119,
        )
    }

    #[test]
    fn test_find_largest_joltage_part_two_example_three() {
        assert_eq!(
            find_largest_joltage("234234234234278".to_string(), RESULT_LENGTH),
            434234234278,
        )
    }

    #[test]
    fn test_find_largest_joltage_part_two_example_four() {
        assert_eq!(
            find_largest_joltage("818181911112111".to_string(), RESULT_LENGTH),
            888911112111,
        )
    }

    #[test]
    fn test_solve_part_two_example() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";

        assert_eq!(solve(input.to_string(), RESULT_LENGTH), 3121910778619,)
    }
}
