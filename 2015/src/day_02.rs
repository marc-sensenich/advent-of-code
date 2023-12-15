use advent_of_code::read_lines;
use log::{debug, log_enabled, Level};
use regex::Regex;
use std::path::Path;

struct GiftBox {
    length: u32,
    width: u32,
    height: u32,
}

impl GiftBox {
    pub fn new(length: u32, width: u32, height: u32) -> GiftBox {
        GiftBox {
            length,
            width,
            height,
        }
    }

    pub fn wrapping_paper_required(&self) -> u32 {
        let intermediaries: Vec<u32> = vec![
            2 * self.length * self.width,
            2 * self.width * self.height,
            2 * self.height * self.length,
        ];

        intermediaries.iter().sum::<u32>() + (intermediaries.iter().min().unwrap() / 2)
    }

    pub fn ribbon_required(&self) -> u32 {
        let intermediaries: Vec<u32> = vec![
            (2 * self.length) + (2 * self.width),
            (2 * self.width) + (2 * self.height),
            (2 * self.height) + (2 * self.length),
        ];
        let ribbon_to_wrap: u32 = *intermediaries.iter().min().unwrap();
        let ribbon_for_bow: u32 = self.length * self.width * self.height;

        ribbon_to_wrap + ribbon_for_bow
    }
}

pub fn part_one(input_path: &Path) -> i32 {
    let mut sum: i32 = 0;

    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(result) = line {
                let re = Regex::new(r"(?<length>\d+)x(?<width>\d+)x(?<height>\d+)").unwrap();
                match re.captures(&result) {
                    Some(capture) => {
                        sum += GiftBox::new(
                            capture["length"].to_string().parse::<u32>().unwrap(),
                            capture["width"].to_string().parse::<u32>().unwrap(),
                            capture["height"].to_string().parse::<u32>().unwrap(),
                        )
                        .wrapping_paper_required() as i32;
                    }
                    None => {}
                }
            }
        }
    }

    sum
}

pub fn part_two(input_path: &Path) -> i32 {
    let mut sum: i32 = 0;

    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(result) = line {
                let re = Regex::new(r"(?<length>\d+)x(?<width>\d+)x(?<height>\d+)").unwrap();
                match re.captures(&result) {
                    Some(capture) => {
                        sum += GiftBox::new(
                            capture["length"].to_string().parse::<u32>().unwrap(),
                            capture["width"].to_string().parse::<u32>().unwrap(),
                            capture["height"].to_string().parse::<u32>().unwrap(),
                        )
                        .ribbon_required() as i32;
                    }
                    None => {}
                }
            }
        }
    }

    sum
}
