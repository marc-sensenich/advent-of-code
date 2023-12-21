use advent_of_code::{read_lines, Coordinate};
use log::{debug, log_enabled, Level};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
use std::fmt::Display;
use std::path::Path;

pub fn part_one(input_path: &Path, goal_distance: u64) -> u64 {
    let mut height_map: HashMap<Coordinate, Item> = HashMap::new();
    let mut starting_node: Option<Item> = None;

    if let Ok(lines) = read_lines(input_path) {
        for (row, line) in lines.enumerate() {
            if let Ok(result) = line {
                for (column, char) in result.chars().enumerate() {
                    let coordinate: Coordinate = Coordinate::new(row as i32, column as i32);
                    let item: Item = Item::new(char, coordinate);
                    height_map.insert(coordinate, item.clone());

                    if char == STARTING_CHAR {
                        starting_node = Some(item.clone());
                    }
                }
            }
        }
    }

    print_board(&height_map);

    let mut result: u64 = u64::MIN;
    if let Some(_starting_node) = starting_node {
        result = running_up_that_hill(&_starting_node, goal_distance, &mut height_map);
    }

    print_board(&height_map);

    result
}

pub fn part_two(input_path: &Path) -> u64 {
    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(result) = line {
                debug!("{}", result);
            }
        }
    }

    0
}

fn running_up_that_hill(
    start: &Item,
    goal_distance: u64,
    map: &mut HashMap<Coordinate, Item>,
) -> u64 {
    let mut queue: VecDeque<Item> = VecDeque::new();
    queue.push_front(start.clone());

    // Distances stores previous distance and current distance to check if the elf gets
    // stuck in a loop ;(
    let mut distances: HashMap<Coordinate, (u64, u64)> = HashMap::with_capacity(map.len());
    distances.insert(start.coordinate, (1, 0));

    while !queue.is_empty() {
        let mut node: Item = queue.pop_front().unwrap();
        for neighbor in node.neighbors(map) {
            let previous_neighbor_distance: u64 =
                distances.get(&neighbor.coordinate).unwrap_or(&(1, 0)).0;
            let current_neighbor_distance: u64 =
                distances.get(&neighbor.coordinate).unwrap_or(&(1, 0)).1;

            if previous_neighbor_distance != goal_distance {
                let current_distance: u64 = distances.get(&node.coordinate).unwrap().1 + 1;

                if current_distance < goal_distance && current_distance != current_neighbor_distance
                {
                    queue.push_back(neighbor.clone());
                }
                distances
                    .entry(neighbor.coordinate.clone())
                    .and_modify(|v| *v = (v.1, current_distance))
                    .or_insert((0, current_distance));
                map.entry(neighbor.coordinate).and_modify(|e| e.value = 'O');
            }
        }
    }

    distances
        .values()
        .copied()
        .filter(|v| v.1 == goal_distance)
        .collect::<Vec<_>>()
        .len() as u64
}

const STARTING_CHAR: char = 'S';

#[derive(Debug, Clone)]
struct Item {
    value: char,
    coordinate: Coordinate,
    neighbors: Option<Vec<Item>>,
}

impl Item {
    pub fn new(value: char, coordinate: Coordinate) -> Item {
        Item {
            value,
            coordinate,
            neighbors: None,
        }
    }

    pub fn can_climb(&self, other: &Item) -> bool {
        other.value != '#'
    }

    pub fn neighbors(&mut self, height_map: &HashMap<Coordinate, Item>) -> Vec<Item> {
        return match &self.neighbors {
            Some(n) => n.to_vec(),
            None => {
                let mut neighbors: Vec<Item> = Vec::new();

                for neighboring_coordinate in self.neighboring_coordinates() {
                    if let Some(neighbor) = height_map.get(&neighboring_coordinate) {
                        if self.can_climb(neighbor) {
                            neighbors.push(neighbor.clone());
                        }
                    }
                }

                self.neighbors = Some(neighbors.clone());
                neighbors
            }
        };
    }

    pub fn neighboring_coordinates(&self) -> Vec<Coordinate> {
        self.coordinate.neighboring_cardinal_directions()
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value,)
    }
}

fn print_board(board: &HashMap<Coordinate, Item>) {
    if log_enabled!(Level::Debug) {
        let x_iter = board.keys().into_iter().map(|c| c.x).into_iter();

        let y_iter = board.keys().into_iter().map(|c| c.y).into_iter();

        let min_x = x_iter.clone().min().unwrap_or(0);
        let max_x = x_iter.max().unwrap_or(0);
        let min_y = y_iter.clone().min().unwrap_or(0);
        let max_y = y_iter.max().unwrap_or(0);

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                if let Some(bi) = board.get(&Coordinate::new(x, y)) {
                    print!("{}", bi);
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!("{:-<1$}", "", (max_x + 5) as usize);
    }
}
