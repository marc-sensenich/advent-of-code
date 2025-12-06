use advent_of_code::read_file_to_string;
use std::path::Path;

fn determine_invalid_ids_part_two(lower: u64, upper: u64) -> Vec<u64> {
    let mut invalid_ids: Vec<u64> = vec![];
    let range = upper - lower;
    for i in 0..=range {
        let comp_value = (lower + i).to_string();
        let mid_point = comp_value.len() / 2;

        for j in 1..=mid_point {
            let pattern: String = comp_value.chars().take(j).collect();
            let remaining_strings = comp_value
                .split(&pattern)
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>();

            if remaining_strings.len() == 0 {
                invalid_ids.push(comp_value.parse::<u64>().unwrap());
                break;
            }
        }
    }

    invalid_ids
}

fn determine_invalid_ids_part_one(mut lower: u64, mut upper: u64) -> Vec<u64> {
    let mut invalid_ids: Vec<u64> = vec![];

    let lower_is_even = (lower.ilog10() + 1) % 2 == 0;
    let upper_is_even = (upper.ilog10() + 1) % 2 == 0;

    let upper_first_half_value: u64;
    let lower_first_half_value: u64;

    if !lower_is_even && !upper_is_even {
        return invalid_ids;
    }

    if !lower_is_even {
        let mut new_lower = vec!["1"];
        new_lower.append(&mut vec!["0"; (lower.ilog10() + 1) as usize]);
        lower = new_lower.concat().parse::<u64>().unwrap();
    }

    if !upper_is_even {
        upper = vec!["9"; upper.ilog10() as usize]
            .concat()
            .parse::<u64>()
            .unwrap();
    }

    let lower_string = lower.to_string();
    let (lower_first_half, lower_second_half) = lower_string.split_at(lower_string.len() / 2);
    let upper_string = upper.to_string();
    let (upper_first_half, upper_second_half) = upper_string.split_at(upper_string.len() / 2);

    lower_first_half_value = lower_first_half.parse::<u64>().unwrap();
    upper_first_half_value = upper_first_half.parse::<u64>().unwrap();

    let lower_second_half_value = lower_second_half.parse::<u64>().unwrap();
    let upper_second_half_value = upper_second_half.parse::<u64>().unwrap();

    if lower_first_half_value == upper_first_half_value {
        if lower_first_half_value >= lower_second_half_value
            && upper_first_half_value <= upper_second_half_value
        {
            invalid_ids.push(
                format!("{}{}", lower_first_half_value, lower_first_half_value)
                    .parse::<u64>()
                    .unwrap(),
            );
        }
    } else {
        if lower_first_half_value >= lower_second_half_value {
            invalid_ids.push(
                format!("{}{}", lower_first_half_value, lower_first_half_value)
                    .parse::<u64>()
                    .unwrap(),
            );
        }

        for x in 1..(upper_first_half_value - lower_first_half_value) {
            invalid_ids.push(
                format!(
                    "{}{}",
                    lower_first_half_value + x,
                    lower_first_half_value + x
                )
                .parse::<u64>()
                .unwrap(),
            )
        }

        if upper_first_half_value <= upper_second_half_value {
            invalid_ids.push(
                format!("{}{}", upper_first_half_value, upper_first_half_value)
                    .parse::<u64>()
                    .unwrap(),
            );
        }
    }

    invalid_ids
}

fn solve_part_one(input: String) -> u64 {
    let mut invalid_ids: Vec<u64> = vec![];
    for line in input.split(",") {
        let input_values = line
            .split("-")
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        invalid_ids.append(&mut determine_invalid_ids_part_one(
            input_values[0],
            input_values[1],
        ));
    }

    invalid_ids.iter().sum::<u64>()
}

fn solve_part_two(input: String) -> u64 {
    let mut invalid_ids: Vec<u64> = vec![];
    for line in input.split(",") {
        let input_values = line
            .split("-")
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        invalid_ids.append(&mut determine_invalid_ids_part_two(
            input_values[0],
            input_values[1],
        ));
    }

    invalid_ids.iter().sum::<u64>()
}

pub fn part_one(input_path: &Path) -> u64 {
    let mut result: u64 = 0;
    if let Ok(input) = read_file_to_string(input_path) {
        result = solve_part_one(input)
    }

    result
}

pub fn part_two(input_path: &Path) -> u64 {
    let mut result: u64 = 0;
    if let Ok(input) = read_file_to_string(input_path) {
        result = solve_part_two(input)
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_range_example_one() {
        assert_eq!(determine_invalid_ids_part_one(11, 22), vec![11, 22],)
    }

    #[test]
    fn test_invalid_range_example_two() {
        assert_eq!(determine_invalid_ids_part_one(99, 115), vec![99],)
    }

    #[test]
    fn test_invalid_range_example_three() {
        assert_eq!(determine_invalid_ids_part_one(998, 1012), vec![1010],)
    }

    #[test]
    fn test_invalid_range_example_four() {
        assert_eq!(
            determine_invalid_ids_part_one(1188511880, 1188511890),
            vec![1188511885],
        )
    }

    #[test]
    fn test_invalid_range_example_five() {
        assert_eq!(determine_invalid_ids_part_one(222220, 222224), vec![222222],)
    }

    #[test]
    fn test_invalid_range_example_six() {
        assert_eq!(determine_invalid_ids_part_one(1698522, 1698528), vec![],)
    }

    #[test]
    fn test_invalid_range_example_seven() {
        assert_eq!(determine_invalid_ids_part_one(446443, 446449), vec![446446],)
    }

    #[test]
    fn test_invalid_range_example_eight() {
        assert_eq!(
            determine_invalid_ids_part_one(38593856, 38593862),
            vec![38593859],
        )
    }

    #[test]
    fn test_invalid_range_example_nine() {
        assert_eq!(
            determine_invalid_ids_part_one(2121212118, 2121212124),
            vec![],
        )
    }

    #[test]
    fn test_invalid_range_example_ten() {
        assert_eq!(
            determine_invalid_ids_part_one(57540345, 57638189),
            vec![
                57545754, 57555755, 57565756, 57575757, 57585758, 57595759, 57605760, 57615761,
                57625762, 57635763
            ],
        )
    }

    #[test]
    fn test_invalid_range_example_eleven() {
        assert_eq!(determine_invalid_ids_part_one(565653, 565659), vec![],)
    }

    #[test]
    fn test_invalid_range_example_twelve() {
        assert_eq!(
            determine_invalid_ids_part_one(4355566991, 4355749498),
            vec![4355643556, 4355743557],
        )
    }

    #[test]
    fn test_invalid_range_example_thirteen() {
        assert_eq!(determine_invalid_ids_part_one(999, 1012), vec![1010],)
    }

    #[test]
    fn test_solve_part_one_example() {
        assert_eq!(
            solve_part_one("11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124".parse().unwrap()),
            1227775554,
        )
    }

    #[test]
    fn test_invalid_range_part_two_example_one() {
        assert_eq!(determine_invalid_ids_part_two(11, 22), vec![11, 22],)
    }

    #[test]
    fn test_invalid_range_part_two_example_five() {
        assert_eq!(determine_invalid_ids_part_two(222220, 222224), vec![222222],)
    }

    #[test]
    fn test_solve_part_two_example() {
        assert_eq!(
            solve_part_two("11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124".parse().unwrap()),
            4174379265,
        )
    }
}
