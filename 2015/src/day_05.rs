use advent_of_code::read_lines;
use log::{debug, log_enabled, Level};
use regex::Regex;
use std::collections::VecDeque;
use std::path::Path;

pub fn part_one(input_path: &Path) -> i32 {
    let mut sum: i32 = 0;
    let letter_twice_in_row_re = Regex::new(
        &('a'..='z')
            .map(|c| format!("{ch}{ch}", ch = c))
            .collect::<Vec<String>>()
            .join("|"),
    )
    .unwrap();
    let vowels_re = Regex::new(r"[aeiou]").unwrap();
    let forbidden_string_re = Regex::new(r"ab|cd|pq|xy").unwrap();
    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(result) = line {
                let contains_three_vowels = vowels_re.find_iter(&result).count() >= 3;
                let letter_twice_in_row = letter_twice_in_row_re.find_iter(&result).count() != 0;
                let contains_no_forbidden_strings =
                    forbidden_string_re.find_iter(&result).count() == 0;
                let is_nice =
                    contains_three_vowels && letter_twice_in_row && contains_no_forbidden_strings;

                debug!("Line: {:?}", result);
                debug!("Contains 3 vowels: {:?}", contains_three_vowels);
                debug!("Contains letter twice in row: {:?}", letter_twice_in_row);
                debug!(
                    "Contains no forbidden string: {:?}",
                    contains_no_forbidden_strings
                );
                debug!("Is nice: {:?}", is_nice);

                if is_nice {
                    sum += 1;
                }
            }
        }
    }

    sum
}

pub fn part_two(input_path: &Path) -> i32 {
    let mut answer: i32 = 0;
    let mut result_chars: VecDeque<char>;
    let mut possible_pair: (Option<char>, Option<char>);
    let mut pair_with_letter_between_chars: Vec<char>;
    let mut contains_pair: bool = false;
    let mut contains_pair_with_letter_between: bool = false;

    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(result) = line {
                result_chars = result.chars().collect();
                possible_pair = (result_chars.pop_front(), result_chars.pop_front());
                pair_with_letter_between_chars = result.chars().collect::<Vec<char>>();
                pair_with_letter_between_chars.sort();
                pair_with_letter_between_chars.dedup();

                for c in pair_with_letter_between_chars {
                    let re = Regex::new(format!(r"{c}\w{{1}}{c}", c = c).as_str()).unwrap();
                    match re.is_match(&result) {
                        false => continue,
                        true => {
                            contains_pair_with_letter_between = true;
                            break;
                        }
                    }
                }

                while !result_chars.is_empty() {
                    let possible_pair_pattern = format!(
                        "{}{}",
                        possible_pair.0.unwrap_or('\0'),
                        possible_pair.1.unwrap_or('\0')
                    );

                    if result_chars
                        .iter()
                        .collect::<String>()
                        .contains(&possible_pair_pattern)
                    {
                        contains_pair = true;
                        break;
                    }

                    possible_pair = (possible_pair.1, result_chars.pop_front())
                }
            }

            if contains_pair && contains_pair_with_letter_between {
                answer += 1;
            }

            contains_pair_with_letter_between = false;
            contains_pair = false;
        }
    }

    answer
}
