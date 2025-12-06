use advent_of_code::read_lines;
use std::path::Path;

pub fn part_one(input_path: &Path) -> i32 {
    let mut password = 0;
    let mut dial = Dial::new(None);
    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(result) = line {
                let (dir, mut _dist) = result.split_at(1);
                let dist;
                if let Ok(num) = _dist.parse::<i32>() {
                    dist = num;
                } else {
                    continue;
                }

                match dir {
                    "R" => dial.rotate_right(dist),
                    "L" => dial.rotate_left(dist),
                    _ => continue,
                };

                if dial.index == 0 {
                    password += 1;
                }
            }
        }
    }

    password
}

pub fn part_two(input_path: &Path) -> i32 {
    let mut password = 0;
    let mut dial = Dial::new(None);
    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(result) = line {
                let (dir, mut _dist) = result.split_at(1);
                let dist;
                if let Ok(num) = _dist.parse::<i32>() {
                    dist = num;
                } else {
                    continue;
                }
                let zero_points: i32;

                match dir {
                    "R" => (_, zero_points) = dial.rotate_right(dist),
                    "L" => (_, zero_points) = dial.rotate_left(dist),
                    _ => continue,
                };

                password += zero_points;
            }
        }
    }

    password
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct Dial {
    indexes: i32,
    pub index: i32,
}

impl Dial {
    pub fn new(indexes: Option<i32>) -> Dial {
        match indexes {
            None => Dial {
                indexes: 100,
                index: 50,
            },
            Some(_indexes) => Dial {
                indexes: _indexes,
                index: _indexes / 2,
            },
        }
    }

    pub fn rotate_right(&mut self, distance: i32) -> (i32, i32) {
        let zero_points: f32 =
            ((self.index as f32 + distance as f32) / self.indexes as f32).floor();
        self.index = ((self.index + distance) % self.indexes + self.indexes) % self.indexes;

        (self.index, zero_points as i32)
    }

    pub fn rotate_left(&mut self, distance: i32) -> (i32, i32) {
        let starting_index = self.index;
        let mut zero_points: f32 = ((self.index as f32 - distance as f32) / self.indexes as f32)
            .floor()
            .abs();

        if starting_index == 0 {
            if zero_points >= 1f32 {
                zero_points -= 1f32;
            }
        }
        self.index = ((self.index - distance) % self.indexes + self.indexes) % self.indexes;
        if self.index == 0 {
            zero_points += 1f32;
        }

        (self.index, zero_points as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_dial_once_left() {
        let mut dial = Dial::new(None);

        assert_eq!(dial.rotate_left(68), (82, 1));
    }

    #[test]
    fn test_rotate_dial_once_right() {
        let mut dial = Dial::new(None);

        assert_eq!(dial.rotate_right(2), (52, 0),);
    }

    #[test]
    fn test_rotate_dial_right_by_1000() {
        let mut dial = Dial::new(None);

        assert_eq!(dial.rotate_right(1000), (50, 10),);
    }

    #[test]
    fn test_rotate_dial_left_by_1000() {
        let mut dial = Dial::new(None);

        assert_eq!(dial.rotate_left(1000), (50, 10),);
    }

    #[test]
    fn test_rotate_dial_from_0_right_by_806() {
        let mut dial = Dial::new(None);
        dial.rotate_right(50);

        assert_eq!(dial.rotate_right(806), (6, 8),);
    }

    #[test]
    fn test_rotate_dial_from_0_left_by_719() {
        let mut dial = Dial::new(None);
        dial.rotate_right(50);

        assert_eq!(dial.rotate_left(719), (81, 7),);
    }

    #[test]
    fn test_rotate_dial_from_0_left_by_30() {
        let mut dial = Dial::new(None);
        dial.rotate_left(50);

        assert_eq!(dial.rotate_left(30), (70, 0),);
    }

    #[test]
    fn test_rotate_dial_from_0_right_by_525() {
        let mut dial = Dial::new(None);
        dial.rotate_left(50);

        assert_eq!(dial.rotate_right(525), (25, 5),);
    }

    #[test]
    fn test_small_dial() {
        let mut dial = Dial::new(Some(6));
        assert_eq!(dial.rotate_left(3), (0, 1));
        assert_eq!(dial.rotate_right(1), (1, 0));
        assert_eq!(dial.rotate_right(13), (2, 2));
        assert_eq!(dial.rotate_right(4), (0, 1));
    }
}
