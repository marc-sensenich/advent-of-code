use advent_of_code::read_lines;
use log::{debug, log_enabled, Level};
use std::path::Path;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Range {
    start: i64,
    end: i64,
    range: i64,
    offset: i64,
}

impl Range {
    pub fn new(start: i64, range: i64, offset: i64) -> Range {
        Range{
            start,
            end: start + range - 1,
            range,
            offset,
        }
    }

    pub fn in_range(&self, val: i64) -> bool {
        val >= self.start && val <= self.end
    }

    pub fn convert_to_destination_value(&self, value: i64) -> i64 {
        value + self.offset
    }
}

fn convert_map_item_to_range(item: &str) -> Range {
    let results: Vec<i64> = item.split_whitespace().collect::<Vec<&str>>().into_iter().map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    let dest_range_start: i64 = *results.get(0).unwrap();
    let src_range_start: i64 = *results.get(1).unwrap();
    let range_length: i64 = *results.get(2).unwrap();

    Range::new(src_range_start, range_length, dest_range_start - src_range_start)
}


pub fn part_one(input_path: &Path) -> i64 {
    let mut minimum_seed_location: i64 = i64::MAX;

    let mapping_order: Vec<String> = vec![
        "seed-to-soil".to_string(),
        "soil-to-fertilizer".to_string(),
        "fertilizer-to-water".to_string(),
        "water-to-light".to_string(),
        "light-to-temperature".to_string(),
        "temperature-to-humidity".to_string(),
        "humidity-to-location".to_string(),
    ];
    let mut mappings: HashMap<String, Vec<Range>> = HashMap::new();
    for mapping_id in &mapping_order {
        mappings.insert(mapping_id.to_string(), vec![]);
    }

    let mut seeds: Vec<i64> = vec![];
    let mut current_input_map: String = "".to_string();

    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(result) = line {
                if result.is_empty() {
                    continue;
                }
                else if result.contains("seeds:") {
                    seeds = result.split(": ").collect::<Vec<&str>>().get(1).unwrap().split_whitespace().collect::<Vec<&str>>().into_iter().map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
                }
                else if result.contains("map:") {
                    current_input_map = result.split(" map:").collect::<Vec<&str>>().get(0).unwrap().trim().to_string();
                }
                else {
                    if let Some(mapping) = mappings.get_mut(&current_input_map) {
                        mapping.push(convert_map_item_to_range(&result));
                    }
                }
            }
        }
    }

    for seed in seeds {
        let mut current_value: i64 = seed;
        for mapping in &mapping_order {
            match mappings.get(mapping) {
                Some(_mapping) => {
                    for m in _mapping {
                        if m.in_range(current_value as i64) {
                            current_value = m.convert_to_destination_value(current_value);
                            break;
                        }
                    }
                }
                None => todo!(),
            }
        }

        if current_value < minimum_seed_location {
            minimum_seed_location = current_value
        }
    }

    minimum_seed_location
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