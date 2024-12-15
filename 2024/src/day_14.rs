use advent_of_code::{read_file_to_string, read_lines, Coordinate};
use log::{debug, log_enabled, Level};
use regex::Regex;
use std::collections::HashMap;
use std::path::Path;

pub fn part_one(input_path: &Path) -> i32 {
    match read_file_to_string(input_path) {
        Ok(input) => solve_part_one(input.as_str(), 100, 101, 103),
        _ => 0,
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

type Velocity = (i32, i32);
struct Robot {
    coordinate: Coordinate,
    velocity: Velocity,
}

impl Robot {
    pub fn new(coordinate: Coordinate, velocity: Velocity) -> Robot {
        Robot {
            coordinate,
            velocity,
        }
    }
}

type Robots = Vec<Robot>;

fn solve_part_one(input: &str, seconds: i32, x_tiles: i32, y_tiles: i32) -> i32 {
    let mut quadrants: HashMap<usize, i32> = HashMap::from([(1, 0), (2, 0), (3, 0), (4, 0)]);

    let x_midpoint = x_tiles / 2;
    let y_midpoint = y_tiles / 2;

    parse_input(input)
        .iter()
        .map(|r| update_position(r, seconds, x_tiles, y_tiles))
        .filter(|c| c.x != x_midpoint && c.y != y_midpoint)
        .for_each(|c| {
            match c.x > x_midpoint {
                // Quadrant 1 or 4
                true => {
                    match c.y < y_midpoint {
                        // Quadrant 1
                        true => {
                            if let Some(v) = quadrants.get_mut(&1) {
                                *v += 1;
                            }
                        }
                        // Quadrant 4
                        false => {
                            if let Some(v) = quadrants.get_mut(&4) {
                                *v += 1;
                            }
                        }
                    }
                }
                // Quadrant 2 or 3
                false => {
                    match c.y < y_midpoint {
                        // Quadrant 2
                        true => {
                            if let Some(v) = quadrants.get_mut(&2) {
                                *v += 1;
                            }
                        }
                        // Quadrant 3
                        false => {
                            if let Some(v) = quadrants.get_mut(&3) {
                                *v += 1;
                            }
                        }
                    }
                }
            }
        });

    quadrants
        .values()
        .map(|i| i.clone())
        .reduce(|acc, e| acc * e)
        .unwrap()
}

fn update_position(robot: &Robot, seconds: i32, x_tiles: i32, y_tiles: i32) -> Coordinate {
    Coordinate::new(
        (robot.coordinate.x + robot.velocity.0 * seconds).rem_euclid(x_tiles),
        (robot.coordinate.y + robot.velocity.1 * seconds).rem_euclid(y_tiles),
    )
}

fn parse_input(input: &str) -> Robots {
    let regex = Regex::new(r"(?m)p=(?<x>\d+),(?<y>\d+)\s+v=(?<vx>-?\d+),(?<vy>-?\d+)").unwrap();

    regex
        .captures_iter(input)
        .map(|capture| {
            Robot::new(
                Coordinate::new(
                    capture["x"].parse::<i32>().unwrap(),
                    capture["y"].parse::<i32>().unwrap(),
                ),
                (
                    capture["vx"].parse::<i32>().unwrap(),
                    capture["vy"].parse::<i32>().unwrap(),
                ),
            )
        })
        .collect::<Robots>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_position_example() {
        let expected: Coordinate = Coordinate::new(1, 3);
        let got: Coordinate =
            update_position(&Robot::new(Coordinate::new(2, 4), (2, -3)), 5, 11, 7);

        assert_eq!(expected, got);
    }

    #[test]
    fn solve_part_one_example() {
        let input: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

        let expected: i32 = 12;
        let got: i32 = solve_part_one(input, 100, 11, 7);

        assert_eq!(got, expected);
    }
}
