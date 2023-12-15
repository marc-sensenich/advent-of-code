use std::io::BufRead;
use advent_of_code::{read_lines, read_file_to_string};
use log::{debug, log_enabled, Level};
use std::path::Path;
use regex::Regex;
use std::collections::HashMap;

fn holiday_ascii_string_helper_algorithm(step: &str) -> u64 {
    let mut current_value: u64 = 0;
    for byte in step.as_bytes() {
        current_value = ((current_value + *byte as u64) * 17) % 256;
    }

    current_value
}

pub fn part_one(input_path: &Path) -> u64 {
    let mut input: String = String::from("");

    if let Ok(_input) = read_file_to_string(input_path) {
        input = _input;
    }

    input
        .split(",")
        .map(|m| holiday_ascii_string_helper_algorithm(m)).sum::<u64>()
}

pub fn part_two(input_path: &Path) -> u64 {
    let mut input: String = String::from("");
    let re = Regex::new(r"(?P<label>[a-z]+)[=-](?P<focal_length>\d+)*").unwrap();
    let mut boxes: HashMap<u64, HashMap<&str, u64>> = HashMap::new();
    let mut boxes_new: HashMap<u64, Vec<(&str, u64)>> = HashMap::new();

    if let Ok(_input) = read_file_to_string(input_path) {
        input = _input;
    }

    for step in input.split(",") {
        let caps = re.captures(step).unwrap();
        let label: &str = match caps.name("label") {
            Some(_label) => _label.as_str(),
            None => "",
        };

        let hashed_label: u64 = holiday_ascii_string_helper_algorithm(label);

        match caps.name("focal_length") {
            Some(focal_length) => {
                let focal_length_u64 = focal_length.as_str().parse::<u64>().unwrap();

                boxes.entry(hashed_label)
                    .and_modify(|_box| {_box.entry(label).and_modify(|fl| *fl = focal_length_u64).or_insert(focal_length_u64);})
                    .or_insert(HashMap::from([(label, focal_length_u64)]));

                boxes_new.entry(hashed_label)
                    .and_modify(|_box| {
                        match _box.iter().position(|b| b.0 == label) {
                            Some(idx) => {
                                _box.remove(idx);
                                _box.insert(idx, (label, focal_length_u64));
                            },
                            None => _box.push((label, focal_length_u64)),
                        }
                    })
                    .or_insert(vec![(label, focal_length_u64)]);
            }
            None => {
                boxes.entry(hashed_label).and_modify(|_box| { _box.remove(label); });
                boxes_new.entry(hashed_label)
                    .and_modify(|_box| {
                        _box.retain(|b| b.0 != label);
                    });
            }
        }
    }

    let mut focusing_power: u64 = 0;

    for _box in boxes_new {
        let box_number: u64 = _box.0 + 1;
        let lenses: Vec<(&str, u64)> = _box.1.clone();
        for (idx, lens) in lenses.iter().enumerate() {
            let slot_number: u64 = (idx + 1) as u64;
            let focal_length: u64 = lens.1;

            focusing_power += box_number * slot_number *focal_length;
        }
    }

    focusing_power
}
