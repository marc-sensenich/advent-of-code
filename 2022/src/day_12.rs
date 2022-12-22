use advent_of_code::{read_lines, Coordinate};
use std::collections::{HashMap, VecDeque};
use std::path::Path;

const STARTING_CHAR: char = 'S';
const GOAL_CHAR: char = 'E';

#[derive(Debug, Copy, Clone)]
struct Item {
    value: char,
    coordinate: Coordinate,
    height: u8,
}

impl Item {
    pub fn new(value: char, coordinate: Coordinate) -> Item {
        let height: u8 = match value {
            STARTING_CHAR => 'a' as u8 - 96,
            GOAL_CHAR => 'z' as u8 - 96,
            _ => value as u8 - 96,
        };

        Item {
            value,
            coordinate,
            height,
        }
    }

    pub fn can_climb(&self, other: &Item) -> bool {
        other.height <= self.height || other.height == (self.height + 1)
    }

    pub fn neighbors(&self, height_map: &HashMap<Coordinate, Item>) -> Vec<Item> {
        let mut neighbors: Vec<Item> = Vec::new();

        for neighboring_coordinate in self.neighboring_coordinates() {
            if let Some(neighbor) = height_map.get(&neighboring_coordinate) {
                if self.can_climb(neighbor) {
                    neighbors.push(neighbor.clone());
                }
            }
        }

        neighbors
    }

    pub fn neighboring_coordinates(&self) -> Vec<Coordinate> {
        self.coordinate.neighboring_cardinal_directions()
    }
}

fn running_up_that_hill(start: &Item, goal: &Item, height_map: &HashMap<Coordinate, Item>) -> i32 {
    if start.coordinate == goal.coordinate {
        return 0;
    }

    let mut queue: VecDeque<Item> = VecDeque::new();
    queue.push_front(start.clone());

    let mut distances: HashMap<Coordinate, i32> = HashMap::with_capacity(height_map.len());
    distances.insert(start.coordinate, 0);

    while !queue.is_empty() {
        let node: Item = queue.pop_front().unwrap();
        for neighbor in node.neighbors(height_map) {
            if distances.get(&neighbor.coordinate).is_none() {
                distances.insert(
                    neighbor.coordinate.clone(),
                    *distances.get(&node.coordinate).unwrap() + 1,
                );
                if neighbor.coordinate != goal.coordinate {
                    queue.push_back(neighbor);
                }
            }
        }
    }

    match distances.get(&goal.coordinate) {
        Some(distance) => *distance,
        None => i32::MAX,
    }
}

pub fn part_one(input_path: &Path) -> i32 {
    let mut height_map: HashMap<Coordinate, Item> = HashMap::new();
    let mut starting_node: Option<Item> = None;
    let mut goal_node: Option<Item> = None;

    if let Ok(lines) = read_lines(input_path) {
        for (row, line) in lines.enumerate() {
            if let Ok(result) = line {
                for (column, char) in result.chars().enumerate() {
                    let coordinate: Coordinate = Coordinate::new(row as i32, column as i32);
                    let item: Item = Item::new(char, coordinate);
                    height_map.insert(coordinate, item);

                    if char == GOAL_CHAR {
                        goal_node = Some(item.clone());
                    }

                    if char == STARTING_CHAR {
                        starting_node = Some(item.clone());
                    }
                }
            }
        }
    }

    if let Some(_starting_node) = starting_node {
        if let Some(_goal_node) = goal_node {
            return running_up_that_hill(&_starting_node, &_goal_node, &height_map);
        }
    }

    -1
}

pub fn part_two(input_path: &Path) -> i32 {
    let mut height_map: HashMap<Coordinate, Item> = HashMap::new();
    let mut possible_starting_nodes: Vec<Item> = Vec::new();
    let mut goal_node: Option<Item> = None;

    if let Ok(lines) = read_lines(input_path) {
        for (row, line) in lines.enumerate() {
            if let Ok(result) = line {
                for (column, char) in result.chars().enumerate() {
                    let coordinate: Coordinate = Coordinate::new(row as i32, column as i32);
                    let item: Item = Item::new(char, coordinate);
                    height_map.insert(coordinate, item);

                    if char == GOAL_CHAR {
                        goal_node = Some(item.clone());
                    }

                    if char == STARTING_CHAR || char == 'a' {
                        possible_starting_nodes.push(item.clone());
                    }
                }
            }
        }
    }

    let mut distances: Vec<i32> = Vec::with_capacity(possible_starting_nodes.len());
    for starting_node in possible_starting_nodes {
        if let Some(_goal_node) = goal_node {
            let distance: i32 = running_up_that_hill(&starting_node, &_goal_node, &height_map);
            distances.push(distance);
        }
    }

    *distances.iter().min().unwrap()
}
