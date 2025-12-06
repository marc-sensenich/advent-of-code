use advent_of_code::{read_file_to_string, Coordinate};
use std::collections::HashMap;
use std::path::Path;

fn solve(diagram: &mut Diagram) -> u64 {
    let mut result: u64 = 0;
    let mut movable_rolls: Vec<Coordinate> = vec![];

    for (coordinate, &val) in diagram.iter() {
        let mut neighboring_rolls_of_paper: u8 = 0;
        if val == '@' {
            for neighbor in coordinate.all_neighboring_directions() {
                match diagram.get(&neighbor) {
                    Some(&c) => {
                        if c == '@' {
                            neighboring_rolls_of_paper += 1
                        }
                    }
                    _ => continue,
                }
            }

            if neighboring_rolls_of_paper < 4 {
                result += 1;
                movable_rolls.push(coordinate.clone());
            }
        }
    }

    for coordinate in movable_rolls {
        if let Some(c) = diagram.get_mut(&coordinate) {
            *c = '.';
        }
    }

    result
}

fn solve_part_two(diagram: &mut Diagram) -> u64 {
    let mut movable_rolls: u64 = solve(diagram);
    let mut result: u64 = movable_rolls;

    while movable_rolls != 0 {
        movable_rolls = solve(diagram);
        result += movable_rolls;
    }

    result
}

pub fn part_one(input_path: &Path) -> u64 {
    let mut result: u64 = 0;
    if let Ok(input) = read_file_to_string(input_path) {
        result = solve(&mut input_to_diagram(input))
    }

    result
}

pub fn part_two(input_path: &Path) -> u64 {
    let mut result: u64 = 0;
    if let Ok(input) = read_file_to_string(input_path) {
        result = solve_part_two(&mut input_to_diagram(input))
    }

    result
}

type Diagram = HashMap<Coordinate, char>;

fn input_to_diagram(input: String) -> Diagram {
    let mut diagram: Diagram = Diagram::new();
    for (x, line) in input.lines().enumerate() {
        for (y, char) in line.chars().enumerate() {
            diagram.insert(Coordinate::new(x as i32, y as i32), char);
        }
    }

    diagram
}

#[cfg(test)]
mod part_one_tests {
    use super::*;

    #[test]
    fn solve_part_one_example() {
        let input: String = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."
            .to_string();

        assert_eq!(solve(&mut input_to_diagram(input)), 13,)
    }
}

#[cfg(test)]
mod part_two_tests {
    use super::*;

    #[test]
    fn solve_part_two_example() {
        let input: String = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."
            .to_string();

        assert_eq!(solve_part_two(&mut input_to_diagram(input)), 43,)
    }
}
