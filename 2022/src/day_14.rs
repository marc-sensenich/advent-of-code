use advent_of_code::read_lines;
use clap::builder::Str;
use log::{debug, log_enabled, Level};
use std::collections::HashMap;
use std::fmt;
use std::path::Path;

pub fn part_one(input_path: &Path) -> i32 {
    run(input_path, false, State::FellIntoAbyss)
}

pub fn part_two(input_path: &Path) -> i32 {
    run(input_path, true, State::Finished)
}

fn run(input_path: &Path, infinite_board: bool, final_state: State) -> i32 {
    let mut board: Board = input_to_board(input_path, infinite_board);
    let sand_entry_coordinate: Coordinate = Coordinate::new(500, 0);

    let mut state: State = State::Start;
    let mut sand_instance: Sand = Sand::new(sand_entry_coordinate);
    let mut instances_of_sand: i32 = 1;

    while state != final_state {
        debug!(
            "Instance {} ({}) of sand is starting to descend",
            instances_of_sand, sand_instance.coordinate,
        );

        state = sand_instance.descend(&mut board, infinite_board);
        debug!(
            "Instance {} of sand is finished descending",
            instances_of_sand
        );

        if sand_instance.coordinate == sand_entry_coordinate {
            debug!("We've reached the top of the cave with sand");
            state = State::Finished;
        }
        sand_instance.coordinate = sand_entry_coordinate;

        instances_of_sand = instances_of_sand + 1;
    }

    count_sand(&board)
}

type Board = HashMap<Coordinate, BoardItem>;

#[derive(Debug)]
enum BoardItem {
    Air(Air),
    Rock(Rock),
    Sand(Sand),
}

#[derive(Debug)]
struct Rock {
    coordinate: Coordinate,
}

impl Rock {
    pub fn new(coordinate: Coordinate) -> Rock {
        Rock { coordinate }
    }
}

impl fmt::Display for Rock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#")
    }
}

#[derive(Debug)]
struct Sand {
    coordinate: Coordinate,
    state: State,
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum State {
    Start,
    Descending,
    FellIntoAbyss,
    FellIntoAbyssLeftSide,
    FellIntoAbyssRightSide,
    Blocked,
    Finished,
}

impl Sand {
    pub fn new(coordinate: Coordinate) -> Sand {
        Sand {
            coordinate,
            state: State::Start,
        }
    }

    fn would_fall_into_the_abyss(&mut self, board: &Board, infinite_board: bool) {
        let left_diag_coordinate: Coordinate =
            Coordinate::new(self.coordinate.y - 1, self.coordinate.x + 1);
        let right_diag_coordinate: Coordinate =
            Coordinate::new(self.coordinate.y + 1, self.coordinate.x + 1);

        if board.get(&left_diag_coordinate).is_none() {
            self.state = State::FellIntoAbyssLeftSide;
            if !infinite_board {
                self.state = State::FellIntoAbyss;
            }
        } else if board.get(&right_diag_coordinate).is_none() {
            self.state = State::FellIntoAbyssRightSide;
            if !infinite_board {
                self.state = State::FellIntoAbyss;
            }
        } else {
            self.state = State::Descending;
        }
    }

    pub fn descend(&mut self, board: &mut Board, infinite_board: bool) -> State {
        debug!("Sand is starting to descend from {}", self.coordinate);
        self.state = State::Descending;
        while self.state == State::Descending {
            self.would_fall_into_the_abyss(board, infinite_board);

            match self.state {
                State::FellIntoAbyssLeftSide => {
                    insert_columns(board, 1, ColumnAdd::Left);
                    self.state = State::Descending;
                }
                State::FellIntoAbyssRightSide => {
                    insert_columns(board, 1, ColumnAdd::Right);
                    self.state = State::Descending;
                }
                State::Descending => {
                    if let Some(next_coordinate) = self.can_descend(board) {
                        self.coordinate = next_coordinate;
                    }
                }
                State::Blocked => {
                    debug!(
                        "Sand is blocked and has completed its descent and has come to rest at {}",
                        self.coordinate
                    );
                    self.state = State::Finished;
                }
                State::FellIntoAbyss => {
                    debug!("Sand has fallen into the abyss");
                    board.insert(self.coordinate, BoardItem::Air(Air::new(self.coordinate)));
                }
                _ => {}
            }

            print_board(&board);
        }

        self.update_board(board);

        return self.state.clone();
    }

    fn update_board(&mut self, board: &mut Board) {
        if self.state != State::FellIntoAbyss {
            board.insert(self.coordinate, BoardItem::Sand(Sand::new(self.coordinate)));
        }
    }

    fn blocked(&mut self, next_coordinate: &Coordinate, board: &Board) -> bool {
        let next_board_item: Option<&BoardItem> = board.get(&next_coordinate);

        debug!("Sand is attempting to descend to {}", next_coordinate);

        match next_board_item {
            Some(BoardItem::Air(_air)) => {
                debug!(
                    "Next board item in my descent to {} is {}, I am not blocked",
                    next_coordinate, _air
                );
                false
            }
            Some(BoardItem::Rock(_rock)) => {
                debug!(
                    "Next board item in my descent to {} is {}, I am blocked",
                    next_coordinate, _rock
                );
                true
            }
            Some(BoardItem::Sand(_sand)) => {
                debug!(
                    "Next board item in my descent to {} is {}, I am blocked",
                    next_coordinate, _sand
                );
                true
            }
            _ => {
                debug!(
                    "Fell into the 'abyss' at {}, adding a new column and trying again",
                    next_coordinate
                );
                true
            }
        }
    }

    fn can_descend(&mut self, board: &Board) -> Option<Coordinate> {
        self.state = State::Blocked;

        for coordinate in Vec::from([
            // Below
            Coordinate::new(self.coordinate.y, self.coordinate.x + 1),
            // Left diaganol
            Coordinate::new(self.coordinate.y - 1, self.coordinate.x + 1),
            // Right diaganol
            Coordinate::new(self.coordinate.y + 1, self.coordinate.x + 1),
        ]) {
            if !self.blocked(&coordinate, board) {
                self.state = State::Descending;
                return Some(coordinate);
            }
        }

        None
    }
}

impl fmt::Display for Sand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "o")
    }
}

#[derive(Debug)]
struct Air {
    coordinate: Coordinate,
}

impl Air {
    pub fn new(coordinate: Coordinate) -> Air {
        Air { coordinate }
    }
}

impl fmt::Display for Air {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, ".")
    }
}

#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.y, self.x,)
    }
}

impl Coordinate {
    pub fn new(y: i32, x: i32) -> Coordinate {
        Coordinate { y, x }
    }

    pub fn coordinates_between(&self, neighbor: &Coordinate) -> Vec<Coordinate> {
        let mut coordinates_between: Vec<Coordinate> = Vec::new();

        let x_differences = (self.x - neighbor.x).abs();
        let y_differences = (self.y - neighbor.y).abs();

        for y_difference in 1..y_differences {
            let mut next_y: i32 = self.y + y_difference;

            if neighbor.y < self.y {
                next_y = self.y - y_difference;
            }

            let next_coordinate: Coordinate = Coordinate::new(next_y, self.x);
            coordinates_between.push(next_coordinate);

            debug!(
                "Adding shared neighbor at {} between {} and {}",
                next_coordinate, self, neighbor
            );
        }

        for x_difference in 1..x_differences {
            let mut next_x: i32 = self.x + x_difference;

            if neighbor.x < self.x {
                next_x = self.x - x_difference;
            }

            let next_coordinate: Coordinate = Coordinate::new(self.y, next_x);
            coordinates_between.push(next_coordinate);

            debug!(
                "Adding shared neighbor at {} between {} and {}",
                next_coordinate, self, neighbor
            );
        }

        coordinates_between
    }
}

fn count_sand(board: &Board) -> i32 {
    board
        .values()
        .into_iter()
        .filter(|bi| match bi {
            BoardItem::Sand(_) => true,
            _ => false,
        })
        .count() as i32
}

fn print_board(board: &Board) {
    if log_enabled!(Level::Debug) {
        let max_x = board_max_x(board);
        let min_y = board_min_y(board);
        let max_y = board_max_y(board);

        for x in 0..=max_x {
            for y in min_y..=max_y {
                if let Some(bi) = board.get(&Coordinate::new(y, x)) {
                    match bi {
                        BoardItem::Air(_bi) => print!("{}", _bi),
                        BoardItem::Rock(_bi) => print!("{}", _bi),
                        BoardItem::Sand(_bi) => print!("{}", _bi),
                    }
                }
            }
            println!("\n");
        }
    }
}

fn input_to_board(input_path: &Path, infinite_board: bool) -> Board {
    let mut board: Board = Board::new();

    let min_x: i32 = 0;
    let mut max_x: i32 = -1;
    let mut min_y: i32 = i32::MAX;
    let mut max_y: i32 = -1;

    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(result) = line {
                let mut result_mut: String = result.clone();
                result_mut.retain(|c| !c.is_whitespace());
                let mut input_coordinates: Vec<Coordinate> = Vec::new();
                for coordinate_input in result_mut.split("->") {
                    let y: i32 = coordinate_input.split(',').nth(0).unwrap().parse().unwrap();
                    let x: i32 = coordinate_input.split(',').last().unwrap().parse().unwrap();

                    if x > max_x {
                        max_x = x;
                    }
                    if y < min_y {
                        min_y = y;
                    }

                    if y > max_y {
                        max_y = y;
                    }
                    let input_coordinate: Coordinate = Coordinate::new(y, x);
                    input_coordinates.push(input_coordinate);
                    debug!("Adding a new rock at {}", input_coordinate);
                    board.insert(
                        input_coordinate,
                        BoardItem::Rock(Rock::new(input_coordinate)),
                    );
                }

                for (index, input_coordinate) in input_coordinates.iter().enumerate() {
                    if let Some(next_coordinate) = input_coordinates.get(index + 1) {
                        for coordinate_between in
                            input_coordinate.coordinates_between(next_coordinate)
                        {
                            board.insert(
                                coordinate_between.clone(),
                                BoardItem::Rock(Rock::new(coordinate_between.clone())),
                            );
                        }
                    }
                }
            }
        }
    }

    if infinite_board {
        max_x = max_x + 2;
    }

    // Fill the rest of the board with air
    for row in min_x..=max_x {
        for col in min_y..=max_y {
            let fill_coordinate: Coordinate = Coordinate::new(col, row);
            if board.get(&fill_coordinate).is_none() {
                if row == max_x && infinite_board {
                    board.insert(fill_coordinate, BoardItem::Rock(Rock::new(fill_coordinate)));
                } else {
                    board.insert(fill_coordinate, BoardItem::Air(Air::new(fill_coordinate)));
                }
            }
        }
    }

    board
}

fn board_min_y(board: &Board) -> i32 {
    board
        .values()
        .into_iter()
        .map(|bi| match bi {
            BoardItem::Air(_bi) => _bi.coordinate.y,
            BoardItem::Rock(_bi) => _bi.coordinate.y,
            BoardItem::Sand(_bi) => _bi.coordinate.y,
        })
        .into_iter()
        .min()
        .unwrap()
}

fn board_max_y(board: &Board) -> i32 {
    board
        .values()
        .into_iter()
        .map(|bi| match bi {
            BoardItem::Air(_bi) => _bi.coordinate.y,
            BoardItem::Rock(_bi) => _bi.coordinate.y,
            BoardItem::Sand(_bi) => _bi.coordinate.y,
        })
        .into_iter()
        .max()
        .unwrap()
}

fn board_min_x(board: &Board) -> i32 {
    board
        .values()
        .into_iter()
        .map(|bi| {
            let max = match bi {
                BoardItem::Air(_bi) => _bi.coordinate.x,
                BoardItem::Rock(_bi) => _bi.coordinate.x,
                BoardItem::Sand(_bi) => _bi.coordinate.x,
            };

            max
        })
        .into_iter()
        .min()
        .unwrap()
}

fn board_max_x(board: &Board) -> i32 {
    board
        .values()
        .into_iter()
        .map(|bi| {
            let max = match bi {
                BoardItem::Air(_bi) => _bi.coordinate.x,
                BoardItem::Rock(_bi) => _bi.coordinate.x,
                BoardItem::Sand(_bi) => _bi.coordinate.x,
            };

            max
        })
        .into_iter()
        .max()
        .unwrap()
}

enum ColumnAdd {
    Left,
    Right,
}
fn insert_columns(board: &mut Board, columns_to_add: i32, column_to_add: ColumnAdd) {
    let min_y: i32 = board_min_y(board);
    let max_y: i32 = board_max_y(board);

    let min_x: i32 = board_min_x(board);
    let max_x: i32 = board_max_x(board);
    debug!("Inserting min X {}", min_x);
    debug!("Inserting max X {}", max_x);
    debug!("Inserting min Y {}", min_y);
    debug!("Inserting max Y {}", max_y);
    let start_column_y: i32;
    let column_y: i32;

    match column_to_add {
        ColumnAdd::Left => {
            column_y = min_y - 1;
            start_column_y = min_y - columns_to_add;
        }
        ColumnAdd::Right => {
            start_column_y = max_y + 1;
            column_y = max_y + columns_to_add;
        }
    };

    for row in 0..=max_x {
        for column in start_column_y..=column_y {
            let fill_coordinate: Coordinate = Coordinate::new(column, row);
            debug!("Adding coordinate: {}", fill_coordinate);
            if row == max_x {
                board.insert(fill_coordinate, BoardItem::Rock(Rock::new(fill_coordinate)));
            } else {
                board.insert(fill_coordinate, BoardItem::Air(Air::new(fill_coordinate)));
            }
        }
    }
}
