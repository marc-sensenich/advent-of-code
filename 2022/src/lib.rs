use core::cmp::Ordering;
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

    pub fn manhattan_distance(&self, other: &Coordinate) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    pub fn neighboring_cardinal_directions(&self) -> Vec<Coordinate> {
        return Vec::from([
            Coordinate::new(self.x + 1, self.y),
            Coordinate::new(self.x - 1, self.y),
            Coordinate::new(self.x, self.y + 1),
            Coordinate::new(self.x, self.y - 1),
        ]);
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

#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Coordinate3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Coordinate3D {
    pub fn new(x: i32, y: i32, z: i32) -> Coordinate3D {
        Coordinate3D { x, y, z }
    }

    pub fn neighboring_coordinates(&self) -> Vec<Coordinate3D> {
        return Vec::from([
            Coordinate3D::new(self.x + 1, self.y, self.z),
            Coordinate3D::new(self.x - 1, self.y, self.z),
            Coordinate3D::new(self.x, self.y + 1, self.z),
            Coordinate3D::new(self.x, self.y - 1, self.z),
            Coordinate3D::new(self.x, self.y, self.z + 1),
            Coordinate3D::new(self.x, self.y, self.z - 1),
        ]);
    }
}

impl fmt::Display for Coordinate3D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x={}, y={}, z={}", self.x, self.y, self.z,)
    }
}
