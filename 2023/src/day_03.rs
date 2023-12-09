use advent_of_code::read_lines;
use advent_of_code::Coordinate;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::path::Path;

const PART_NUMBER_REGEX_STR: &str = r"\d+";
const SYMBOL_REGEX_STR: &str = r"[^\d.\n]";
const GEAR_SYMBOL_REGEX_STR: &str = r"\*";

#[derive(Debug)]
struct PossiblePartNumber {
    value: i32,
    neighboring_coordinates: Vec<Coordinate>,
}

#[derive(Debug)]
struct Gear {
    adjacent_part_numbers: Vec<i32>,
}

fn extract_part_numbers_from_line(line: &str, row_index: i32) -> Vec<PossiblePartNumber> {
    let mut possible_part_numbers: Vec<PossiblePartNumber> = vec![];

    let part_number_regex = Regex::new(PART_NUMBER_REGEX_STR).unwrap();
    let part_number_captures = part_number_regex.captures_iter(&line);
    for mat in part_number_captures {
        let mat_value = mat.get(0).unwrap();
        let mut possible_part_number = PossiblePartNumber {
            value: mat_value.as_str().to_string().parse::<i32>().unwrap(),
            neighboring_coordinates: vec![],
        };

        // Calculate the coordinates surrounding the possible
        // part number by calculating the adjacent coordinates

        // Add left and right edges of neighboring coordinates
        for i in -1..=1 {
            possible_part_number
                .neighboring_coordinates
                .push(Coordinate {
                    x: row_index + i,
                    y: mat_value.start() as i32 - 1,
                });
            possible_part_number
                .neighboring_coordinates
                .push(Coordinate {
                    x: row_index + i,
                    y: mat_value.end() as i32,
                });
        }

        // Add neighboring coordinates above and below each number
        for i in mat_value.start()..=mat_value.end() - 1 {
            possible_part_number
                .neighboring_coordinates
                .push(Coordinate {
                    x: row_index - 1,
                    y: i as i32,
                });
            possible_part_number
                .neighboring_coordinates
                .push(Coordinate {
                    x: row_index + 1,
                    y: i as i32,
                });
        }

        possible_part_numbers.push(possible_part_number);
    }

    possible_part_numbers
}

pub fn part_one(input_path: &Path) -> i32 {
    let mut sum: i32 = 0;
    let symbol_regex = Regex::new(SYMBOL_REGEX_STR).unwrap();
    let mut possible_part_numbers: Vec<PossiblePartNumber> = vec![];
    let mut symbol_coordinates: HashSet<Coordinate> = HashSet::new();

    let mut row_index: i32 = 0;

    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(result) = line {
                possible_part_numbers
                    .extend(extract_part_numbers_from_line(&result, row_index).into_iter());
                let symbol_captures = symbol_regex.captures_iter(&result);

                for mat in symbol_captures {
                    let mat_value = mat.get(0).unwrap();
                    symbol_coordinates.insert(Coordinate::new(row_index, mat_value.start() as i32));
                }
            }

            row_index += 1;
        }
    }

    for possible_part_number in &possible_part_numbers {
        for neighboring_coordinate in &possible_part_number.neighboring_coordinates {
            if symbol_coordinates.contains(neighboring_coordinate) {
                sum += possible_part_number.value;
                break;
            }
        }
    }

    sum
}

pub fn part_two(input_path: &Path) -> i32 {
    let mut sum: i32 = 0;
    let symbol_regex = Regex::new(GEAR_SYMBOL_REGEX_STR).unwrap();
    let mut possible_part_numbers: Vec<PossiblePartNumber> = vec![];
    let mut gears: HashMap<Coordinate, Gear> = HashMap::new();

    let mut row_index: i32 = 0;

    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(result) = line {
                possible_part_numbers
                    .extend(extract_part_numbers_from_line(&result, row_index).into_iter());

                let symbol_captures = symbol_regex.captures_iter(&result);
                for mat in symbol_captures {
                    let mat_value = mat.get(0).unwrap();
                    gears.insert(
                        Coordinate::new(row_index, mat_value.start() as i32),
                        Gear {
                            adjacent_part_numbers: vec![],
                        },
                    );
                }
            }

            row_index += 1;
        }
    }

    for possible_part_number in &possible_part_numbers {
        for neighboring_coordinate in &possible_part_number.neighboring_coordinates {
            if gears.contains_key(neighboring_coordinate) {
                if let Some(c) = gears.get_mut(&neighboring_coordinate) {
                    c.adjacent_part_numbers.push(possible_part_number.value);
                }
            }
        }
    }

    for (_, gear) in gears.iter_mut() {
        if gear.adjacent_part_numbers.len() == 2 {
            let gear_ratio: i32 = gear.adjacent_part_numbers.pop().unwrap()
                * gear.adjacent_part_numbers.pop().unwrap();
            sum += gear_ratio;
        }
    }

    sum
}
