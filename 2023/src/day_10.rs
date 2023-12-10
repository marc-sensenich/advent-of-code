use advent_of_code::read_lines;
use advent_of_code::Coordinate;
use log::{debug, log_enabled, Level};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::path::Path;

type Board = HashMap<Coordinate, Tile>;

#[derive(Debug, Clone)]
struct Cursor {
    current_tile: Tile,
    previous_tile: Tile,
    steps: u64,
}

// SqRt( | x2 - x1|^2 + | y2 - y1 | ^2)

impl Cursor {
    pub fn new(starting_tile: Tile, previous_tile: Tile) -> Cursor {
        Cursor {
            current_tile: starting_tile,
            previous_tile,
            steps: 1,
        }
    }

    pub fn increment_step(&mut self) {
        self.steps += 1;
    }

    pub fn next(&mut self, board: Board) {
        let previous_coordinate: Coordinate = self.previous_tile.coordinate;
        let next_coordinate: Coordinate = *self
            .current_tile
            .neighboring_coordinates
            .iter()
            .copied()
            .filter(|c| *c != previous_coordinate)
            .collect::<Vec<Coordinate>>()
            .get(0)
            .unwrap();
        self.previous_tile = self.current_tile.clone();
        self.current_tile = board.get(&next_coordinate).unwrap().clone();
        self.increment_step();

        debug!("Next coordinate: {:#?}", next_coordinate);
    }
}

pub fn part_one(input_path: &Path) -> u64 {
    let mut starting_coordinate: Coordinate = Coordinate::new(0, 0);
    let mut board: Board = Board::new();

    if let Ok(lines) = read_lines(input_path) {
        for (x, line) in lines.enumerate() {
            if let Ok(result) = line {
                for (y, result_char) in result.chars().enumerate() {
                    let tile_coordinate: Coordinate = Coordinate::new(x as i32, y as i32);
                    let tile: Tile = Tile::new(result_char, tile_coordinate.clone());
                    if result_char == 'S' {
                        starting_coordinate = tile_coordinate.clone();
                    }

                    board.insert(tile_coordinate.clone(), tile.clone());
                }
            }
        }
    }

    let starting_tile_actual_symbol: char =
        determine_starting_tile_symbol(board.clone(), starting_coordinate);
    debug!("Starting tile symbol: {}", starting_tile_actual_symbol);
    board
        .entry(starting_coordinate)
        .and_modify(|t| *t = Tile::new(starting_tile_actual_symbol, t.coordinate));
    let starting_tile: Tile = board.get(&starting_coordinate).unwrap().clone();

    let mut cursor_one: Cursor = Cursor::new(
        board
            .get(&starting_tile.neighboring_coordinates[0])
            .unwrap()
            .clone(),
        starting_tile.clone(),
    );
    debug!("Cursor one: {:#?}", cursor_one);
    let mut cursor_two: Cursor = Cursor::new(
        board
            .get(&starting_tile.neighboring_coordinates[1])
            .unwrap()
            .clone(),
        starting_tile.clone(),
    );
    debug!("Cursor two: {:#?}", cursor_two);

    let mut current_distance: i32 = cursor_one
        .current_tile
        .coordinate
        .distance(starting_coordinate);
    let mut previous_distance: i32 = current_distance;

    while cursor_one.current_tile.coordinate != starting_coordinate {
        cursor_one.next(board.clone());
    }

    debug!("Cursor one: {:#?}", cursor_one);

    cursor_one.steps / 2
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

#[derive(Debug, Clone)]
enum TileKind {
    Vertical,
    Horizontal,
    NinetyDegreeNE,
    NinetyDegreeNW,
    NinetyDegreeSW,
    NinetyDegreeSE,
    Start,
    Ground,
    Unknown,
}

#[derive(Debug, Clone)]
struct Tile {
    coordinate: Coordinate,
    symbol: char,
    kind: TileKind,
    neighboring_coordinates: Vec<Coordinate>,
}

impl Tile {
    pub fn new(symbol: char, coordinate: Coordinate) -> Tile {
        let kind: TileKind = Tile::determine_tile_kind(symbol);
        let neighboring_coordinates: Vec<Coordinate> =
            Tile::determine_neighboring_coordinates(kind.clone(), coordinate);

        Tile {
            coordinate,
            symbol,
            kind,
            neighboring_coordinates,
        }
    }

    fn determine_neighboring_coordinates(
        kind: TileKind,
        coordinate: Coordinate,
    ) -> Vec<Coordinate> {
        match kind {
            TileKind::Vertical => {
                [coordinate.north_coordinate(), coordinate.south_coordinate()].to_vec()
            }
            TileKind::Horizontal => {
                [coordinate.east_coordinate(), coordinate.west_coordinate()].to_vec()
            }
            TileKind::NinetyDegreeNE => {
                [coordinate.north_coordinate(), coordinate.east_coordinate()].to_vec()
            }
            TileKind::NinetyDegreeNW => {
                [coordinate.north_coordinate(), coordinate.west_coordinate()].to_vec()
            }
            TileKind::NinetyDegreeSW => {
                [coordinate.south_coordinate(), coordinate.west_coordinate()].to_vec()
            }
            TileKind::NinetyDegreeSE => {
                [coordinate.south_coordinate(), coordinate.east_coordinate()].to_vec()
            }
            _ => vec![],
        }
    }

    fn determine_tile_kind(symbol: char) -> TileKind {
        match symbol {
            '|' => TileKind::Vertical,
            '-' => TileKind::Horizontal,
            'L' => TileKind::NinetyDegreeNE,
            'J' => TileKind::NinetyDegreeNW,
            '7' => TileKind::NinetyDegreeSW,
            'F' => TileKind::NinetyDegreeSE,
            '.' => TileKind::Ground,
            'S' => TileKind::Start,
            _ => TileKind::Unknown,
        }
    }
}

fn determine_starting_tile_symbol(board: Board, starting_tile_coordinate: Coordinate) -> char {
    // To determine type of starting tile
    // North MUST BE Vertical or South East or West
    // South MUST BE Vertical or North East or West
    // East MUST BE horizontal or North West or South West
    // West MUST BE horizontal or North East or South East
    let mut valid_neighbor_cardinal_directions: Vec<char> = vec![];

    debug!("Determining starting tile type");
    if let Some(neighbor_tile) = board.get(&starting_tile_coordinate.north_coordinate()) {
        debug!("There's a northern neighbor on the board");
        match neighbor_tile.kind {
            TileKind::Vertical | TileKind::NinetyDegreeSE | TileKind::NinetyDegreeSW => {
                valid_neighbor_cardinal_directions.push('N')
            }
            _ => debug!("Not a valid northern neighbor kind"),
        }
    }

    if let Some(neighbor_tile) = board.get(&starting_tile_coordinate.south_coordinate()) {
        debug!("There's a southern neighbor on the board");
        match neighbor_tile.kind {
            TileKind::Vertical | TileKind::NinetyDegreeNE | TileKind::NinetyDegreeNW => {
                valid_neighbor_cardinal_directions.push('S')
            }
            _ => debug!("Not a valid southern neighbor kind"),
        }
    }

    if let Some(neighbor_tile) = board.get(&starting_tile_coordinate.east_coordinate()) {
        debug!("There's a eastern neighbor on the board");
        match neighbor_tile.kind {
            TileKind::Horizontal | TileKind::NinetyDegreeNW | TileKind::NinetyDegreeSW => {
                valid_neighbor_cardinal_directions.push('E')
            }
            _ => debug!("Not a valid eastern neighbor kind"),
        }
    }

    if let Some(neighbor_tile) = board.get(&starting_tile_coordinate.west_coordinate()) {
        debug!("There's a western neighbor on the board");
        match neighbor_tile.kind {
            TileKind::Horizontal | TileKind::NinetyDegreeNE | TileKind::NinetyDegreeSE => {
                valid_neighbor_cardinal_directions.push('W')
            }
            _ => debug!("Not a valid western neighbor kind"),
        }
    }

    // North and South Vertical NS
    // North and East, NE
    // North and West, NW
    // South and East, SE
    // South and West, SW
    // East and West, Horizontal EW
    match valid_neighbor_cardinal_directions
        .iter()
        .collect::<String>()
        .as_str()
    {
        // "NS" => TileKind::Vertical,
        "NS" => '|',
        // "EW" => TileKind::Horizontal,
        "EW" => '-',
        // "NE" => TileKind::NinetyDegreeNE,
        "NE" => 'L',
        // "NW" => TileKind::NinetyDegreeNW,
        "NW" => 'J',
        // "SW" => TileKind::NinetyDegreeSW,
        "SW" => '7',
        // "SE" => TileKind::NinetyDegreeSE,
        "SE" => 'F',
        _ => todo!(),
    }
}
