use std::fmt::format;
use advent_of_code::read_lines;
use log::{debug, log_enabled, Level};
use std::path::Path;
use regex::Regex;
use std::collections::HashMap;

pub fn part_one(input_path: &Path) -> i32 {
    let mut sum: i32 = 0;

    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(result) = line {
                let mut scratchcard_value: i32 = 0;

                let parts = result.split(" | ").collect::<Vec<_>>();
                let winning_numbers = parts.get(0).unwrap().split(":").collect::<Vec<_>>().get(1).unwrap().trim().split(" ").filter(|n| !n.is_empty()).map(|n| format!(r"\b{}\b", n)).collect::<Vec<_>>();
                let winning_numbers_re_string =  winning_numbers.join("|");
                let winning_numbers_re = Regex::new(&winning_numbers_re_string).unwrap();
                let my_numbers = parts.get(1).unwrap();
                let my_winning_numbers: Vec<&str> = winning_numbers_re.find_iter(&my_numbers).map(|m| m.as_str()).collect();

                for _ in my_winning_numbers {
                    match scratchcard_value {
                        0 => scratchcard_value = 1,
                        _ => scratchcard_value *= 2,
                    }
                }

                sum += scratchcard_value;
            }
        }
    }

    sum
}

pub fn part_two(input_path: &Path) -> i32 {
    let mut scratchcard_inventory: HashMap<i32, i32> = HashMap::new();
    let mut max_card_number: i32 = 0;

    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(result) = line {
                let parts = result.split(" | ").collect::<Vec<_>>();
                let card_number = parts.get(0).unwrap().split(":").collect::<Vec<_>>().get(0).unwrap().split("Card").collect::<Vec<_>>().get(1).unwrap().trim().parse::<i32>().unwrap();
                if card_number > max_card_number {
                    max_card_number = card_number;
                }
                let _ = *scratchcard_inventory.entry(card_number).or_insert(1);

                let winning_numbers = parts.get(0).unwrap().split(":").collect::<Vec<_>>().get(1).unwrap().trim().split(" ").filter(|n| !n.is_empty()).map(|n| format!(r"\b{}\b", n)).collect::<Vec<_>>();
                let winning_numbers_re_string =  winning_numbers.join("|");
                let winning_numbers_re = Regex::new(&winning_numbers_re_string).unwrap();
                let my_numbers = parts.get(1).unwrap();
                let my_winning_numbers: Vec<&str> = winning_numbers_re.find_iter(&my_numbers).map(|m| m.as_str()).collect();
                let my_winning_numbers_count = my_winning_numbers.len() as i32 ;

                let extra_card_multiplier:i32 = match scratchcard_inventory.get(&card_number) {
                    Some(n) => *n,
                    None => 1,
                };

                for extra_card_number in card_number+1..=card_number+my_winning_numbers_count {
                    *scratchcard_inventory.entry(extra_card_number).or_insert(1) += 1 * extra_card_multiplier;
                }
            }
        }
    }

    // Remove any possible scratch cards from the inventory that have a number greater than the max
    // card given as input.
    for x in max_card_number+1..=*scratchcard_inventory.keys().max().unwrap() as i32 {
        scratchcard_inventory.remove(&x);
    }

    scratchcard_inventory.values().sum::<i32>()
}
