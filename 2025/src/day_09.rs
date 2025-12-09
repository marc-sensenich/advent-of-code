use advent_of_code::{read_file_to_string, read_lines, Coordinate};
use log::{debug, log_enabled, Level};
use std::path::Path;

fn solve_part_one(coordinates: &mut Vec<Coordinate>) -> u64 {
    coordinates.sort();
    let mut max_area: u64 = 0;

    for i in 0..coordinates.len() {
        let c1: Coordinate = coordinates[i];

        for j in (0..(coordinates.len() - i)).rev() {
            let c2: Coordinate = coordinates[j];
            let area: u64 = ((c1.x - c2.x + 1).abs() as i64 * (c1.y - c2.y + 1).abs() as i64) as u64;

            if area > max_area {
                max_area = area;
            }
        }
    }

    max_area
}

fn parse_input(input: String) -> Vec<Coordinate> {
    input.lines().map(|l| {
        let split = l.split(",").filter_map(|x| x.parse::<i32>().ok()).collect::<Vec<i32>>();
        Coordinate::new(split[0], split[1])
    }).collect::<Vec<Coordinate>>()
}

pub fn part_one(input_path: &Path) -> u64 {
    match read_file_to_string(input_path) {
        Ok(input) => {
            solve_part_one(&mut parse_input(input))
        },
        Err(_) => 0,
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

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    fn example_coordinates() -> Vec<Coordinate> {
        vec![
            Coordinate::new(7,1),
            Coordinate::new(11,1),
            Coordinate::new(11,7),
            Coordinate::new(9,7),
            Coordinate::new(9,5),
            Coordinate::new(2,5),
            Coordinate::new(2,3),
            Coordinate::new(7,3),
        ]
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input(EXAMPLE_INPUT.to_string()),
            example_coordinates(),
        )
    }

    #[test]
    fn test_solve_part_one_example() {
        assert_eq!(
            solve_part_one(&mut example_coordinates()),
            50
        )
    }
}
