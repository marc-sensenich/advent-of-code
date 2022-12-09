use advent_of_code::read_lines;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::path::Path;

const UP_DIRECTION_STRING: &str = "U";
const DOWN_DIRECTION_STRING: &str = "D";
const LEFT_DIRECTION_STRING: &str = "L";
const RIGHT_DIRECTION_STRING: &str = "R";

pub fn part_one(input_path: &Path) -> i32 {
    run(input_path, 2)
}

pub fn part_two(input_path: &Path) -> i32 {
    run(input_path, 10)
}

fn run(input_path: &Path, length_of_rope: usize) -> i32 {
    let moves: Vec<Move> = read_input(input_path);
    let mut tail_visited_coordinates: HashSet<Coordinate> = HashSet::new();
    let previous_knot: &mut Knot = &mut Knot::new(-1, Coordinate::new(-1, -1));

    let mut knots: HashMap<usize, Knot> = HashMap::new();
    for knot_id in 0..length_of_rope {
        knots.insert(
            knot_id,
            Knot::new(knot_id.try_into().unwrap(), Coordinate::new(0, 0)),
        );
    }

    for move_to_execute in &moves {
        for _ in 0..move_to_execute.steps {
            // Move the head knot first
            knots.entry(0).and_modify(|k| {
                k.move_in_direction(&move_to_execute.direction, 1);
                *previous_knot = k.clone();
            });
            for current_knot_index in 1..length_of_rope {
                knots.entry(current_knot_index).and_modify(|k| {
                    while !k.touching(&previous_knot) {
                        let knot_moves: Vec<Move> = k.determine_next_moves(&previous_knot);
                        for knot_move in knot_moves {
                            k.move_in_direction(&knot_move.direction, 1);
                        }
                    }
                    *previous_knot = k.clone();
                    if current_knot_index == (length_of_rope - 1) {
                        tail_visited_coordinates.insert(k.coordinate.clone());
                    }
                });
            }
        }
    }

    tail_visited_coordinates.len().try_into().unwrap()
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Unknown,
}

#[derive(Debug)]
struct Move {
    direction: Direction,
    steps: i32,
}

impl Move {
    pub fn new(direction: Direction, steps: i32) -> Move {
        Move { direction, steps }
    }
}

fn line_to_move(line: &String) -> Move {
    let mut split_line = line.split(" ").collect::<Vec<_>>();
    let steps: i32 = split_line
        .pop()
        .expect("unable to pop a move from the input line")
        .parse()
        .unwrap();
    let direction = split_line
        .pop()
        .expect("unable to pop a move from the input line");

    // Use shadowing here to override the direction string from the input
    let direction = match direction {
        UP_DIRECTION_STRING => Direction::Up,
        DOWN_DIRECTION_STRING => Direction::Down,
        LEFT_DIRECTION_STRING => Direction::Left,
        RIGHT_DIRECTION_STRING => Direction::Right,
        &_ => Direction::Unknown,
    };

    Move::new(direction, steps)
}

#[derive(Clone, Copy, Debug)]
struct Knot {
    id: i32,
    coordinate: Coordinate,
}

impl fmt::Display for Knot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Knot '{}' is at {}, {}",
            self.id, self.coordinate.x, self.coordinate.y
        )
    }
}

impl Knot {
    pub fn new(id: i32, coordinate: Coordinate) -> Knot {
        Knot { id, coordinate }
    }

    pub fn touching(&self, knot: &Knot) -> bool {
        // Example: 2,2
        let neighbor_coordinates: HashSet<Coordinate> = HashSet::from([
            // Overlap: 2,2
            Coordinate::new(self.coordinate.x, self.coordinate.y),
            // N: 3,2
            Coordinate::new(self.coordinate.x + 1, self.coordinate.y),
            // S: 1,2
            Coordinate::new(self.coordinate.x - 1, self.coordinate.y),
            // E: 2, 3
            Coordinate::new(self.coordinate.x, self.coordinate.y + 1),
            // W: 2, 1
            Coordinate::new(self.coordinate.x, self.coordinate.y - 1),
            // NW: 3, 1
            Coordinate::new(self.coordinate.x + 1, self.coordinate.y - 1),
            // NE: 3, 3
            Coordinate::new(self.coordinate.x + 1, self.coordinate.y + 1),
            // SW: 1, 1
            Coordinate::new(self.coordinate.x - 1, self.coordinate.y - 1),
            // SE: 1, 2
            Coordinate::new(self.coordinate.x - 1, self.coordinate.y + 1),
        ]);

        neighbor_coordinates.contains(&knot.coordinate)
    }

    pub fn determine_next_moves(&self, knot: &Knot) -> Vec<Move> {
        let x_difference: i32 = self.coordinate.x - knot.coordinate.x;
        let y_difference = self.coordinate.y - knot.coordinate.y;
        let mut moves: Vec<Move> = Vec::new();

        // If the same row, towards the other knot by 1 in the direction
        if x_difference < 0 {
            moves.push(Move::new(Direction::Up, 1));
        } else if x_difference > 0 {
            moves.push(Move::new(Direction::Down, 1));
        }

        // If the same column, towards the other knot by 1 in the direction
        if y_difference < 0 {
            moves.push(Move::new(Direction::Right, 1));
        } else if y_difference > 0 {
            moves.push(Move::new(Direction::Left, 1))
        }

        moves
    }

    pub fn move_in_direction(&mut self, direction: &Direction, steps: i32) {
        match direction {
            Direction::Up => {
                self.coordinate.x = self.coordinate.x + steps;
            }
            Direction::Down => {
                self.coordinate.x = self.coordinate.x - steps;
            }
            Direction::Left => {
                self.coordinate.y = self.coordinate.y - steps;
            }
            Direction::Right => {
                self.coordinate.y = self.coordinate.y + steps;
            }
            Direction::Unknown => {}
        }
    }
}

fn read_input(input_path: &Path) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();

    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(result) = line {
                moves.push(line_to_move(&result));
            }
        }
    }

    moves
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    pub fn new(x: i32, y: i32) -> Coordinate {
        Coordinate { x, y }
    }
}
