use advent_of_code::read_lines;
use log::{debug, log_enabled, Level};
use std::path::Path;
use std::cmp::min;
use std::iter::zip;

pub fn part_one(input_path: &Path) -> u64 {
    let mut rows: Vec<String> = vec![];
    let mut columns: Vec<String> = vec![];
    let mut sum: u64 = 0;

    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(result) = line {
                debug!("{}", result);
                if result.is_empty() {
                    debug!("Split between entry points");
                    sum += determine_reflection_result(&rows, &columns);

                    rows.clear();
                    columns.clear();

                } else {
                    rows.push(String::from(&result));
                    for (idx, c) in result.chars().enumerate() {
                        if let Some(elem) = columns.get_mut(idx) {
                            *elem = elem.clone() + String::from(c).as_str();
                        } else {
                            columns.push(String::from(c));
                        }
                    }
                }
            }
        }
    }

    sum += determine_reflection_result(&rows, &columns);

    sum
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

fn determine_reflection_result_inner(line: &Vec<String>, summary_fn: fn(u64) -> u64) -> u64 {
    let mut sum: u64 = 0;

    for (idx, item) in line.iter().enumerate() {
        match line.get(idx + 1) {
            Some(next_item) => {
                if item == next_item {
                    let items_to_check_left_side = idx;
                    let items_to_check_right_side = line.len() - idx - 2;
                    let items_to_check: usize = min(items_to_check_left_side, items_to_check_right_side);

                    let mut found_reflection: bool = true;

                    for check_idx in 1..=items_to_check {
                        let left_item_to_check = line.get(idx - check_idx);
                        let right_item_to_check = line.get(idx + 1 + check_idx);

                        if left_item_to_check != right_item_to_check {
                            found_reflection = false;
                            break;
                        }
                    }

                    if found_reflection {
                        sum += summary_fn((idx + 1) as u64);
                        break;
                    }
                }
            }
            _ => {},
        }
    }

    sum
}
fn determine_reflection_result(rows: &Vec<String>, columns: &Vec<String>) -> u64 {
    determine_reflection_result_inner(columns, |x| { x })
        + determine_reflection_result_inner(rows, |x| { x * 100 })
}