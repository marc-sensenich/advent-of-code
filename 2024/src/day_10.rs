use advent_of_code::{read_file_to_string, read_lines, Coordinate};
use log::{debug, log_enabled, Level};
use std::collections::{HashMap, HashSet};
use std::path::Path;

pub fn part_one(input_path: &Path) -> u64 {
    match read_file_to_string(input_path) {
        Ok(input) => solve_part_one(input.as_str()),
        _ => 0,
    }
}

pub fn part_two(input_path: &Path) -> u64 {
    match read_file_to_string(input_path) {
        Ok(input) => solve_part_two(input.as_str()),
        _ => 0,
    }
}

type Grid = HashMap<Coordinate, GridItem>;

#[derive(Debug, Clone)]
struct GridItem {
    coordinate: Coordinate,
    visited_neighbors: Vec<Coordinate>,
    value: u32,
}

impl GridItem {
    pub fn new(x: i32, y: i32, value: u32) -> GridItem {
        GridItem {
            coordinate: Coordinate::new(x, y),
            visited_neighbors: vec![],
            value,
        }
    }
}

fn solve_part_one(input: &str) -> u64 {
    let mut result: u64 = 0;
    let mut visited_peaks: HashMap<Coordinate, u64> = HashMap::new();
    let (mut grid, starting_points) = input_to_grid(input);

    for starting_point in starting_points {
        traverse(&mut grid, starting_point, &mut visited_peaks);

        result += visited_peaks.len() as u64;
        visited_peaks.clear();
    }

    result
}

#[cfg(test)]
mod solve_part_one_tests {
    use super::solve_part_one;

    #[test]
    fn simple() {
        let input: &str = &"...0...\n...1...\n...2...\n6543456\n7.....7\n8.....8\n9.....9";
        assert_eq!(solve_part_one(input), 2);
    }

    #[test]
    fn multiple_trails() {
        let input: &str = &"..90..9\n...1.98\n...2..7\n6543456\n765.987\n876....\n987....";
        assert_eq!(solve_part_one(input), 4);
    }

    #[test]
    fn two_trailheads() {
        let input: &str = &"10..9..\n2...8..\n3...7..\n4567654\n...8..3\n...9..2\n.....01";
        assert_eq!(solve_part_one(input), 3);
    }

    #[test]
    fn larger_example() {
        let input: &str =
            &"89010123\n78121874\n87430965\n96549874\n45678903\n32019012\n01329801\n10456732";
        assert_eq!(solve_part_one(input), 36);
    }
}

fn solve_part_two(input: &str) -> u64 {
    let mut result: u64 = 0;
    let mut visited_peaks: HashMap<Coordinate, u64> = HashMap::new();
    let (mut grid, starting_points) = input_to_grid(input);

    for starting_point in starting_points {
        traverse(&mut grid, starting_point, &mut visited_peaks);

        result += visited_peaks.values().sum::<u64>();
        visited_peaks.clear();
    }

    result
}

#[cfg(test)]
mod solve_part_two_tests {
    use super::solve_part_two;

    #[test]
    fn single_trailhead_simple() {
        let input: &str = &".....0.\n..4321.\n..5..2.\n..6543.\n..7..4.\n..8765.\n..9....";
        assert_eq!(solve_part_two(input), 3);
    }

    #[test]
    fn single_trailhead_moderate() {
        let input: &str = &"..90..9\n...1.98\n...2..7\n6543456\n765.987\n876....\n987....";
        assert_eq!(solve_part_two(input), 13);
    }

    #[test]
    fn single_trailhead_complex() {
        let input: &str = &"012345\n123456\n234567\n345678\n4.6789\n56789.";
        assert_eq!(solve_part_two(input), 227);
    }

    #[test]
    fn larger() {
        let input: &str =
            &"89010123\n78121874\n87430965\n96549874\n45678903\n32019012\n01329801\n10456732";
        assert_eq!(solve_part_two(input), 81);
    }
}

fn input_to_grid(input: &str) -> (Grid, Vec<GridItem>) {
    let mut starting_points: Vec<GridItem> = vec![];
    let mut grid: Grid = Grid::new();

    for (x, line) in input.lines().enumerate() {
        for (y, char) in line.chars().enumerate() {
            if let Some(value) = char.to_digit(10) {
                if value == 0 {
                    starting_points.push(GridItem::new(x as i32, y as i32, value));
                }
                grid.insert(
                    Coordinate::new(x as i32, y as i32),
                    GridItem::new(x as i32, y as i32, value),
                );
            }
        }
    }

    (grid, starting_points)
}

fn traverse(board: &mut Grid, start: GridItem, visited_peaks: &mut HashMap<Coordinate, u64>) {
    for neighbor_coordinate in start.coordinate.neighboring_cardinal_directions() {
        if let Some(neighbor) = board.get(&neighbor_coordinate) {
            if neighbor.value > start.value {
                if neighbor.value - start.value == 1 {
                    if neighbor.value == 9 {
                        let visits = visited_peaks.entry(neighbor_coordinate).or_insert(0);
                        *visits += 1;
                    } else {
                        traverse(&mut board.clone(), neighbor.clone(), visited_peaks);
                    }
                }
            }
        }
    }
}
