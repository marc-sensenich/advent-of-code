use advent_of_code::read_lines;
use std::path::Path;

pub fn part_one(input_path: &Path) -> i32 {
    let mut sum: i32 = 0;
    let mut max: i32 = 0;

    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(result) = line {
                if !result.is_empty() {
                    sum += result.parse::<i32>().unwrap();
                } else {
                    if sum > max {
                        max = sum;
                    }

                    sum = 0;
                }
            }
        }
    }

    max
}

pub fn part_two(input_path: &Path) -> i32 {
    let mut sum: i32 = 0;
    let mut calorie_totals: Vec<i32> = Vec::new();

    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(result) = line {
                if !result.is_empty() {
                    let calories = result.parse::<i32>().unwrap();
                    sum += calories;
                } else {
                    calorie_totals.push(sum);

                    sum = 0;
                }
            }
        }
    }

    calorie_totals.sort();
    calorie_totals.reverse();
    calorie_totals.truncate(3);

    calorie_totals.iter().sum::<i32>()
}
