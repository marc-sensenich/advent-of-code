use advent_of_code::read_lines;
use log::{debug, log_enabled, Level};
use std::path::Path;

pub fn part_one(input_path: &Path) -> i32 {
    let mut safe_reports: i32 = 0;

    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(result) = line {
                if is_safe_report(&result, false) {
                    safe_reports += 1;
                }
            }
        }
    }

    safe_reports
}

pub fn part_two(input_path: &Path) -> i32 {
    let mut safe_reports: i32 = 0;

    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(result) = line {
                if is_safe_report(&result, true) {
                    safe_reports += 1;
                }
            }
        }
    }

    safe_reports
}

fn safe_report_check(report_items: Vec<&str>) -> bool {
    let mut report_results: Vec<i32> = vec![];
    let expected_report_results = report_items.len() - 1;

    for w in report_items.windows(2) {
        report_results.push(w[0].parse::<i32>().unwrap() - w[1].parse::<i32>().unwrap());
    }

    let only_decreasing_report_results = report_results
        .clone()
        .into_iter()
        .filter(|x| x > &0)
        .collect::<Vec<_>>();
    let all_levels_are_decreasing: bool =
        only_decreasing_report_results.len() == expected_report_results;

    let only_increasing_report_results = report_results
        .clone()
        .into_iter()
        .filter(|x| x < &0)
        .collect::<Vec<_>>();
    let all_levels_are_increasing: bool =
        only_increasing_report_results.len() == expected_report_results;

    let only_report_results_in_range = report_results
        .clone()
        .into_iter()
        .filter(|x| x.abs() <= 3 && x.abs() != 0)
        .collect::<Vec<_>>();
    let all_levels_are_within_range: bool =
        only_report_results_in_range.len() == expected_report_results;

    (all_levels_are_decreasing || all_levels_are_increasing) && all_levels_are_within_range
}

fn is_safe_report(report: &str, dampener_enabled: bool) -> bool {
    let report_items = report.split_whitespace().collect::<Vec<_>>();

    match dampener_enabled {
        true => {
            let mut safe_report: bool = safe_report_check(report_items.clone());

            if !safe_report {
                for x in 0..report_items.len() {
                    let mut dampened_report_items = report_items.clone();
                    dampened_report_items.remove(x);

                    safe_report = safe_report_check(dampened_report_items);
                    if safe_report {
                        break;
                    }
                }
            }

            safe_report
        }
        false => safe_report_check(report_items),
    }
}
