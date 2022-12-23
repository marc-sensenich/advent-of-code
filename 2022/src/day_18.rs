use advent_of_code::{read_lines, Coordinate, Coordinate3D};
use std::collections::HashSet;
use std::path::Path;

pub fn part_one(input_path: &Path) -> i32 {
    let surface_coordinates: HashSet<Coordinate3D> = input_to_coordinates(input_path);
    let mut uncovered_sides: u32 = 0;

    for surface_coordinate in &surface_coordinates {
        let neighbors = surface_coordinate.neighboring_coordinates();
        for neighbor in &neighbors {
            if !surface_coordinates.contains(neighbor) {
                uncovered_sides = uncovered_sides + 1;
            }
        }
    }

    uncovered_sides as i32
}

pub fn part_two(input_path: &Path) -> i32 {
    0
}

fn input_to_coordinates(input_path: &Path) -> HashSet<Coordinate3D> {
    let mut coordinates: HashSet<Coordinate3D> = HashSet::new();

    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(result) = line {
                let coordinate_value: Vec<i32> = result
                    .split(",")
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();

                coordinates.insert(Coordinate3D::new(
                    coordinate_value[0],
                    coordinate_value[1],
                    coordinate_value[2],
                ));
            }
        }
    }

    coordinates
}
