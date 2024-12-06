use std::cmp::Ordering;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// read_lines from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_file_to_string<P>(filename: P) -> Result<String, std::io::Error>
where
    P: AsRef<Path>,
{
    std::fs::read_to_string(filename)
}

#[derive(Clone, Copy, Ord, Eq, PartialEq, Hash, Debug)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

impl Coordinate {
    pub fn new(x: i32, y: i32) -> Coordinate {
        Coordinate { x, y }
    }

    pub fn neighboring_cardinal_directions(&self) -> Vec<Coordinate> {
        return Vec::from([
            self.north_coordinate(),
            self.south_coordinate(),
            self.east_coordinate(),
            self.west_coordinate(),
        ]);
    }

    pub fn north_coordinate(&self) -> Coordinate {
        Coordinate::new(self.x - 1, self.y)
    }

    pub fn south_coordinate(&self) -> Coordinate {
        Coordinate::new(self.x + 1, self.y)
    }

    pub fn east_coordinate(&self) -> Coordinate {
        Coordinate::new(self.x, self.y + 1)
    }

    pub fn west_coordinate(&self) -> Coordinate {
        Coordinate::new(self.x, self.y - 1)
    }
}

impl PartialOrd for Coordinate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.x == other.x {
            return Some(self.y.cmp(&other.y));
        }
        Some(self.y.cmp(&other.y))
    }
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x={}, y={}", self.x, self.y,)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
    Unknown,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Direction::North => 'N',
                Direction::South => 'S',
                Direction::East => 'E',
                Direction::West => 'W',
                Direction::Unknown => '?',
            }
        )
    }
}
