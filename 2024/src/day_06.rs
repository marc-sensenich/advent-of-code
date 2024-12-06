use std::collections::{HashMap, HashSet};
use advent_of_code::{read_lines, Coordinate, Direction};
use log::{debug, log_enabled, Level};
use std::path::Path;


type Board = HashMap<Coordinate, char>;

pub fn part_one(input_path: &Path) -> i32 {
    let mut visited: HashSet<Coordinate> = HashSet::new();
    let mut cursor: Coordinate = Coordinate::new(-1, -1);
    let mut direction_of_travel: Direction = Direction::North;
    let mut board: Board = Board::new();
    let mut left_the_building: bool = false;

    if let Ok(lines) = read_lines(input_path) {
        for (x, line) in lines.enumerate() {
            if let Ok(result) = line {
                for (y, c) in result.chars().enumerate() {
                    board.insert(Coordinate::new(x as i32, y as i32), c);
                    if c == '^' {
                        cursor = Coordinate::new(x as i32, y as i32);
                    }
                }
            }
        }
    }

    while !left_the_building {
        visited.insert(cursor);
        match direction_of_travel {
            Direction::North => {
                match board.get(&cursor.north_coordinate()) {
                    Some(c) => {
                        if c == &'#' {
                            direction_of_travel = Direction::East;
                            cursor = cursor.east_coordinate();
                        } else {
                            cursor = cursor.north_coordinate();
                        }
                    }
                    None => left_the_building = true
                }
            }
            Direction::South => {
                match board.get(&cursor.south_coordinate()) {
                    Some(c) => {
                        if c == &'#' {
                            direction_of_travel = Direction::West;
                            cursor = cursor.west_coordinate();
                        } else {
                            cursor = cursor.south_coordinate();
                        }
                    }
                    None => left_the_building = true
                }
            }
            Direction::East => {
                match board.get(&cursor.east_coordinate()) {
                    Some(c) => {
                        if c == &'#' {
                            direction_of_travel = Direction::South;
                            cursor = cursor.south_coordinate();
                        } else {
                            cursor = cursor.east_coordinate();
                        }
                    }
                    None => left_the_building = true
                }
            }
            Direction::West => {
                match board.get(&cursor.west_coordinate()) {
                    Some(c) => {
                        if c == &'#' {
                            direction_of_travel = Direction::North;
                            cursor = cursor.north_coordinate();
                        } else {
                            cursor = cursor.west_coordinate();
                        }
                    }
                    None => left_the_building = true
                }
            }
            Direction::Unknown => {

            }
        }
    }

    visited.len() as i32
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
