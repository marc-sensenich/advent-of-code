use advent_of_code::{read_lines, Coordinate};
use log::{debug, log_enabled, Level};
use std::collections::{HashMap, HashSet, VecDeque};
use std::path::Path;

pub fn part_one(input_path: &Path) -> u64 {
    traverse(&mut input_to_board(input_path), 0, 0, Direction::East)
}

pub fn part_two(input_path: &Path) -> u64 {
    let mut result: u64 = 0;
    let mut board: Board = input_to_board(input_path);
    let mut results: Vec<u64> = vec![];

    let max_x = board
        .values()
        .into_iter()
        .map(|bi| bi.coordinate.x)
        .into_iter()
        .max()
        .unwrap();
    let max_y = board
        .values()
        .into_iter()
        .map(|bi| bi.coordinate.y)
        .into_iter()
        .max()
        .unwrap();

    let mut directions_to_traverse: Vec<Direction> = vec![];

    for x in 0..=max_x {
        for y in 0..=max_y {
            if x == 0 && y == 0 {
                directions_to_traverse.push(Direction::East);
                directions_to_traverse.push(Direction::South);
            } else if x == 0 && y == (max_y) {
                directions_to_traverse.push(Direction::South);
                directions_to_traverse.push(Direction::West);
            } else if x == (max_x) && y == 0 {
                directions_to_traverse.push(Direction::North);
                directions_to_traverse.push(Direction::East);
            } else if x == (max_x) && y == (max_y) {
                directions_to_traverse.push(Direction::North);
                directions_to_traverse.push(Direction::West);
            } else if x == 0 {
                directions_to_traverse.push(Direction::South);
            } else if y == 0 {
                directions_to_traverse.push(Direction::East);
            } else if x == (max_x) {
                directions_to_traverse.push(Direction::North);
            } else if y == (max_y) {
                directions_to_traverse.push(Direction::West);
            }

            for direction in directions_to_traverse.iter().cloned() {
                results.push(traverse(&mut board.clone(), x, y, direction));
            }

            directions_to_traverse.clear();
        }
    }

    *results.iter().max().unwrap_or(&0)
}

fn traverse(board: &mut Board, x: i32, y: i32, starting_direction: Direction) -> u64 {
    let mut direction: Direction = starting_direction;
    let mut queue: VecDeque<BoardItem> = VecDeque::new();
    let mut direction_queue: VecDeque<Direction> = VecDeque::new();

    let mut next: BoardItem;
    if let Some(bi) = board.get(&Coordinate::new(x, y)) {
        queue.push_back(bi.clone());
    }
    direction_queue.push_back(direction);

    let mut neighbor_coordinates: Vec<(Coordinate, Direction)> = vec![];

    while !queue.is_empty() {
        next = queue.pop_front().unwrap();
        direction = direction_queue.pop_front().unwrap();

        next.energized = true;
        next.visits += 1;

        match next.symbol {
            '.' => match direction {
                Direction::North => {
                    neighbor_coordinates.push((
                        Coordinate::new(next.coordinate.x - 1, next.coordinate.y),
                        Direction::North,
                    ));
                }
                Direction::South => {
                    neighbor_coordinates.push((
                        Coordinate::new(next.coordinate.x + 1, next.coordinate.y),
                        Direction::South,
                    ));
                }
                Direction::East => {
                    neighbor_coordinates.push((
                        Coordinate::new(next.coordinate.x, next.coordinate.y + 1),
                        Direction::East,
                    ));
                }
                Direction::West => {
                    neighbor_coordinates.push((
                        Coordinate::new(next.coordinate.x, next.coordinate.y - 1),
                        Direction::West,
                    ));
                }
            },
            '|' => match direction {
                Direction::North => {
                    neighbor_coordinates.push((
                        Coordinate::new(next.coordinate.x - 1, next.coordinate.y),
                        Direction::North,
                    ));
                }
                Direction::South => {
                    neighbor_coordinates.push((
                        Coordinate::new(next.coordinate.x + 1, next.coordinate.y),
                        Direction::South,
                    ));
                }
                Direction::East | Direction::West => {
                    neighbor_coordinates.push((
                        Coordinate::new(next.coordinate.x - 1, next.coordinate.y),
                        Direction::North,
                    ));
                    neighbor_coordinates.push((
                        Coordinate::new(next.coordinate.x + 1, next.coordinate.y),
                        Direction::South,
                    ));
                }
                _ => {}
            },
            '-' => match direction {
                Direction::North | Direction::South => {
                    neighbor_coordinates.push((
                        Coordinate::new(next.coordinate.x, next.coordinate.y - 1),
                        Direction::West,
                    ));
                    neighbor_coordinates.push((
                        Coordinate::new(next.coordinate.x, next.coordinate.y + 1),
                        Direction::East,
                    ));
                }
                Direction::East => {
                    neighbor_coordinates.push((
                        Coordinate::new(next.coordinate.x, next.coordinate.y + 1),
                        Direction::East,
                    ));
                }
                Direction::West => {
                    neighbor_coordinates.push((
                        Coordinate::new(next.coordinate.x, next.coordinate.y - 1),
                        Direction::West,
                    ));
                }
            },
            '/' => match direction {
                Direction::North => {
                    neighbor_coordinates.push((
                        Coordinate::new(next.coordinate.x, next.coordinate.y + 1),
                        Direction::East,
                    ));
                }
                Direction::South => {
                    neighbor_coordinates.push((
                        Coordinate::new(next.coordinate.x, next.coordinate.y - 1),
                        Direction::West,
                    ));
                }
                Direction::East => {
                    neighbor_coordinates.push((
                        Coordinate::new(next.coordinate.x - 1, next.coordinate.y),
                        Direction::North,
                    ));
                }
                Direction::West => {
                    neighbor_coordinates.push((
                        Coordinate::new(next.coordinate.x + 1, next.coordinate.y),
                        Direction::South,
                    ));
                }
            },
            '\\' => match direction {
                Direction::North => {
                    neighbor_coordinates.push((
                        Coordinate::new(next.coordinate.x, next.coordinate.y - 1),
                        Direction::West,
                    ));
                }
                Direction::South => {
                    neighbor_coordinates.push((
                        Coordinate::new(next.coordinate.x, next.coordinate.y + 1),
                        Direction::East,
                    ));
                }
                Direction::East => {
                    neighbor_coordinates.push((
                        Coordinate::new(next.coordinate.x + 1, next.coordinate.y),
                        Direction::South,
                    ));
                }
                Direction::West => {
                    neighbor_coordinates.push((
                        Coordinate::new(next.coordinate.x - 1, next.coordinate.y),
                        Direction::North,
                    ));
                }
            },
            _ => {}
        }

        while let Some(nc) = neighbor_coordinates.pop() {
            let next_index: Coordinate = Coordinate::new(nc.0.x, nc.0.y);
            let bi: Option<&BoardItem> = board.get(&next_index);

            match bi {
                Some(_bi) => {
                    let mut __bi = _bi.clone();
                    __bi.visits += 1;
                    __bi.energized = true;
                    if !next.visited_neighbors.contains(&next_index) {
                        next.visited_neighbors.push(next_index);
                        queue.push_back(__bi.clone());
                        direction_queue.push_back(nc.1);
                    }

                    board.insert(__bi.coordinate, __bi.clone());
                    board.insert(next.coordinate, next.clone());
                }
                None => {}
            }
        }
    }

    board.values().filter(|bi| bi.energized).count() as u64
}

type Board = HashMap<Coordinate, BoardItem>;

#[derive(Debug, Clone)]
struct BoardItem {
    coordinate: Coordinate,
    energized: bool,
    visits: u32,
    symbol: char,
    visited_neighbors: Vec<Coordinate>,
}

impl BoardItem {
    pub fn new(x: i32, y: i32, symbol: char) -> BoardItem {
        BoardItem {
            coordinate: Coordinate::new(x, y),
            energized: false,
            symbol,
            visits: 0,
            visited_neighbors: vec![],
        }
    }
}

fn input_to_board(input_path: &Path) -> Board {
    let mut board: Board = Board::new();

    if let Ok(lines) = read_lines(input_path) {
        for (x, line) in lines.enumerate() {
            if let Ok(result) = line {
                for (y, char) in result.chars().enumerate() {
                    board.insert(
                        Coordinate::new(x as i32, y as i32),
                        BoardItem::new(x as i32, y as i32, char),
                    );
                }
            }
        }
    }

    board
}

#[derive(Debug, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}
