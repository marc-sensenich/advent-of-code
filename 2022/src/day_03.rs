use advent_of_code::read_lines;
use std::collections::{HashMap, HashSet};
use std::path::Path;

pub fn part_one(input_path: &Path) -> i32 {
    let priorities = generate_priorities();
    let mut total = 0;

    if let Ok(lines) = read_lines(input_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(result) = line {
                let mut first_compartment: String = result.clone();
                let compartment_length = first_compartment.trim().len() / 2;

                let second_compartment: String = first_compartment.split_off(compartment_length);

                let first_compartment_set: HashSet<char> =
                    HashSet::from_iter(first_compartment.chars());
                let second_compartment_set: HashSet<char> =
                    HashSet::from_iter(second_compartment.chars());

                let intersection = first_compartment_set.intersection(&second_compartment_set);
                match intersection.last() {
                    Some(duplicate) => {
                        if let Some(value) = priorities.get(&duplicate) {
                            total += *value;
                        }
                    }
                    None => println!("No duplicate found in the compartments"),
                }
            }
        }
    }

    total
}

pub fn part_two(input_path: &Path) -> i32 {
    let priorities = generate_priorities();
    let mut total = 0;

    if let Ok(lines) = read_lines(input_path) {
        // Consumes the iterator, returns an (Optional) String
        let mut peekable_lines = lines.peekable();
        while peekable_lines.peek().is_some() {
            let rucksack_one: HashSet<char> =
                HashSet::from_iter(peekable_lines.next().unwrap().unwrap().chars());
            let rucksack_two: HashSet<char> =
                HashSet::from_iter(peekable_lines.next().unwrap().unwrap().chars());
            let rucksack_three: HashSet<char> =
                HashSet::from_iter(peekable_lines.next().unwrap().unwrap().chars());

            let badge_set_one: HashSet<&char> =
                HashSet::from_iter(rucksack_one.intersection(&rucksack_two));
            let badge_set_two: HashSet<&char> =
                HashSet::from_iter(rucksack_two.intersection(&rucksack_three));

            let badge_set_final = badge_set_one.intersection(&badge_set_two);

            match badge_set_final.last() {
                Some(badge) => {
                    if let Some(value) = priorities.get(&badge) {
                        total += *value;
                    }
                }
                None => println!("No duplicate badge found"),
            }
        }
    }

    total
}

fn generate_priorities() -> HashMap<char, i32> {
    let mut priorities: HashMap<char, i32> = HashMap::new();
    // Character iteration from https://users.rust-lang.org/t/iteration-thought-alphabets/30078/4
    let letters = b'a'..=b'z';
    for (i, c) in letters.enumerate() {
        let char_c = c as char;
        priorities.insert(char_c, (i + 1).try_into().unwrap());
        priorities.insert(char_c.to_ascii_uppercase(), (i + 27).try_into().unwrap());
    }

    priorities
}
