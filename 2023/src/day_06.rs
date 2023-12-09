use advent_of_code::read_lines;
use log::debug;
use regex::Regex;
use std::iter::zip;
use std::path::Path;

pub fn part_one(input_path: &Path) -> u64 {
    let mut solution: u64 = 1;
    let re = Regex::new(r"\d+").unwrap();

    let mut times: Vec<u64> = vec![];
    let mut distances: Vec<u64> = vec![];

    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(result) = line {
                let results: Vec<u64> = re
                    .find_iter(&result)
                    .map(|m| m.as_str().parse::<u64>().unwrap())
                    .collect::<Vec<_>>();

                if result.starts_with("Time") {
                    times = results.clone();
                } else if result.starts_with("Distance") {
                    distances = results.clone();
                }
            }
        }
    }

    let races = zip(times, distances);

    for race in races {
        let current_record_distance: u64 = race.1;
        let race_time: u64 = race.0;
        let mut record_distances: u64 = 0;

        for t in 0..=race_time {
            let possible_distance = t * (race_time - t);
            if possible_distance > current_record_distance {
                record_distances += 1;
            }
        }

        debug!("Possible record distances: {}", record_distances);
        if record_distances > 0 {
            solution *= record_distances;
        }
    }

    solution
}

pub fn part_two(input_path: &Path) -> u64 {
    let mut solution: u64 = 1;
    let re = Regex::new(r"\d+").unwrap();

    let mut times: Vec<u64> = vec![];
    let mut distances: Vec<u64> = vec![];

    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(result) = line {
                let results: Vec<u64> = re
                    .find_iter(&result.replace(" ", ""))
                    .map(|m| m.as_str().parse::<u64>().unwrap())
                    .collect::<Vec<_>>();

                if result.starts_with("Time") {
                    times = results.clone();
                } else if result.starts_with("Distance") {
                    distances = results.clone();
                }
            }
        }
    }

    let races = zip(times, distances);

    for race in races {
        debug!("{:#?} {:#?}", race.0, race.1);
        let current_record_distance: u64 = race.1;
        let race_time: u64 = race.0;
        let mut record_distances: u64 = 0;

        for t in 0..=race_time {
            let possible_distance = t * (race_time - t);
            if possible_distance > current_record_distance {
                record_distances += 1;
            }
        }

        if record_distances > 0 {
            solution *= record_distances;
        }
    }

    solution
}
