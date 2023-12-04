use advent_of_code::{read_lines,read_file_to_string,Coordinate};
use log::{debug, log_enabled, Level};
use std::path::Path;
use std::collections::{HashSet, HashMap};


pub fn part_one(input_path: &Path) -> i32 {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut visited_locations: HashSet<Coordinate> = HashSet::new();

    visited_locations.insert(Coordinate::new(x, y));

    if let Ok(input) = read_file_to_string(input_path) {
        for c in input.chars() {
            match c {
                '>' => {
                    y += 1;
                }
                '<' => {
                    y -= 1;
                }
                '^' => {
                    x -= 1;
                }
                'v' => {
                    x += 1;
                }
                _ => todo!()
            }

            visited_locations.insert(Coordinate::new(x, y));
        }
    }

    visited_locations.len() as i32
}

pub fn part_two(input_path: &Path) -> i32 {
    let mut santa_x: i32 = 0;
    let mut santa_y: i32 = 0;
    let mut robo_santa_x: i32 = 0;
    let mut robo_santa_y: i32 = 0;
    let mut visited_locations: HashSet<Coordinate> = HashSet::new();

    visited_locations.insert(Coordinate::new(santa_x, santa_y));
    visited_locations.insert(Coordinate::new(robo_santa_x, robo_santa_y));
    let mut robo_santa_move: bool = false;

    if let Ok(input) = read_file_to_string(input_path) {
        for (i, c) in input.chars().enumerate() {
            match c {
                '>' => {
                    match robo_santa_move {
                        true => robo_santa_y += 1,
                        false => santa_y += 1,
                    }
                }
                '<' => {
                    match robo_santa_move {
                        true => robo_santa_y -= 1,
                        false => santa_y -= 1,
                    }
                }
                '^' => {
                    match robo_santa_move {
                        true => robo_santa_x -= 1,
                        false => santa_x -= 1,
                    }
                }
                'v' => {
                    match robo_santa_move {
                        true => robo_santa_x += 1,
                        false => santa_x += 1,
                    }
                }
                _ => todo!()
            }

            match robo_santa_move {
                true => visited_locations.insert(Coordinate::new(robo_santa_x, robo_santa_y)),
                false => visited_locations.insert(Coordinate::new(santa_x, santa_y)),
            };

            robo_santa_move = !robo_santa_move;
        }
    }

    visited_locations.len() as i32
}
