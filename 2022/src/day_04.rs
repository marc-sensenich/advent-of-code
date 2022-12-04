use advent_of_code::read_lines;
use std::collections::HashSet;
use std::path::Path;

pub fn part_one(input_path: &Path) -> i32 {
    let mut overlaps: i32 = 0;

    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(result) = line {
                let (first_section, second_section) = parse_section_assignments(result);
                if first_section.is_superset(&second_section)
                    || first_section.is_subset(&second_section)
                {
                    overlaps += 1;
                }
            }
        }
    }

    overlaps
}

pub fn part_two(input_path: &Path) -> i32 {
    let mut overlaps: i32 = 0;

    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(result) = line {
                let (first_section, second_section) = parse_section_assignments(result);

                let intersections = first_section.intersection(&second_section).count();
                if intersections > 0 {
                    overlaps += 1;
                }
            }
        }
    }

    overlaps
}

fn parse_section_assignments(result: String) -> (HashSet<i32>, HashSet<i32>) {
    let sections: Vec<&str> = result.split(',').collect();
    let first_section_vec: Vec<i32> = sections
        .first()
        .unwrap()
        .split('-')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    let second_section_vec: Vec<i32> = sections
        .last()
        .unwrap()
        .split('-')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let first_section: HashSet<i32> = HashSet::from_iter(
        *first_section_vec.first().unwrap()..=*first_section_vec.last().unwrap(),
    );
    let second_section: HashSet<i32> = HashSet::from_iter(
        *second_section_vec.first().unwrap()..=*second_section_vec.last().unwrap(),
    );

    (first_section, second_section)
}
