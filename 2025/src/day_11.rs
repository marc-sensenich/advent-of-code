use advent_of_code::read_file_to_string;
use std::collections::{HashMap, VecDeque};
use std::path::Path;

fn solve_part_one(
    start: Option<String>,
    thing: &mut HashMap<String, VecDeque<String>>,
    visited: &mut VecDeque<String>,
) -> VecDeque<VecDeque<String>> {
    let mut result: VecDeque<VecDeque<String>> = VecDeque::new();

    let _start = start.unwrap_or_else(|| "you".to_string());
    if visited.is_empty() {
        visited.push_back(_start.clone());
    }

    match _start == "out" {
        true => {
            let mut blah = VecDeque::new();
            blah.push_back(visited.clone());
            blah
        }
        false => {
            let mut foo = thing.clone();
            let start_vec = foo.get_mut(&_start).unwrap();

            while !start_vec.is_empty() {
                match start_vec.pop_front() {
                    Some(s) => {
                        if !visited.contains(&s) || _start != *s {
                            visited.push_back(s.clone());
                            result.append(&mut solve_part_one(Some(s.clone()), thing, visited));
                            visited.pop_back();
                        }
                    }
                    None => {}
                }
            }

            result
        }
    }
}

fn solve_part_two(
    thing: &mut HashMap<String, VecDeque<String>>,
    visited: &mut VecDeque<String>,
) -> usize {
    // Too slow
    // solve_part_one(Some("svr".to_string()), thing, visited).iter().filter(|&m| m.contains(&"dac".to_string()) && m.contains(&"fft".to_string())).count()
    0
}

fn parse_input(input: String) -> HashMap<String, VecDeque<String>> {
    let mut result: HashMap<String, VecDeque<String>> = HashMap::new();

    for line in input.lines() {
        let split_line: Vec<&str> = line.split(":").collect();
        let device = split_line[0].to_string();
        let outputs = split_line[1]
            .trim()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<VecDeque<String>>();

        result.insert(device, outputs);
    }

    result
}

pub fn part_one(input_path: &Path) -> usize {
    match read_file_to_string(input_path) {
        Ok(input) => solve_part_one(None, &mut parse_input(input), &mut VecDeque::new()).len(),
        Err(_) => 0,
    }
}

pub fn part_two(input_path: &Path) -> usize {
    match read_file_to_string(input_path) {
        Ok(input) => solve_part_two(&mut parse_input(input), &mut VecDeque::new()),
        Err(_) => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT_PART_ONE: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    const EXAMPLE_INPUT_PART_TWO: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

    #[test]
    fn test_solve_part_one() {
        assert_eq!(
            5,
            solve_part_one(
                None,
                &mut parse_input(EXAMPLE_INPUT_PART_ONE.to_string()),
                &mut VecDeque::new()
            )
            .len()
        )
    }

    // #[test]
    fn test_solve_part_two() {
        assert_eq!(
            2,
            solve_part_two(
                &mut parse_input(EXAMPLE_INPUT_PART_TWO.to_string()),
                &mut VecDeque::new()
            )
        )
    }

    #[test]
    fn test_parse_input_example() {
        assert_eq!(
            expected_input(),
            parse_input(EXAMPLE_INPUT_PART_ONE.to_string())
        )
    }

    fn expected_input() -> HashMap<String, VecDeque<String>> {
        let mut expected: HashMap<String, VecDeque<String>> = HashMap::new();
        expected.insert(
            "aaa".to_string(),
            VecDeque::from_iter(vec!["you".to_string(), "hhh".to_string()]),
        );
        expected.insert(
            "you".to_string(),
            VecDeque::from_iter(vec!["bbb".to_string(), "ccc".to_string()]),
        );
        expected.insert(
            "bbb".to_string(),
            VecDeque::from_iter(vec!["ddd".to_string(), "eee".to_string()]),
        );
        expected.insert(
            "ccc".to_string(),
            VecDeque::from_iter(vec![
                "ddd".to_string(),
                "eee".to_string(),
                "fff".to_string(),
            ]),
        );
        expected.insert(
            "ddd".to_string(),
            VecDeque::from_iter(vec!["ggg".to_string()]),
        );
        expected.insert(
            "eee".to_string(),
            VecDeque::from_iter(vec!["out".to_string()]),
        );
        expected.insert(
            "fff".to_string(),
            VecDeque::from_iter(vec!["out".to_string()]),
        );
        expected.insert(
            "ggg".to_string(),
            VecDeque::from_iter(vec!["out".to_string()]),
        );
        expected.insert(
            "hhh".to_string(),
            VecDeque::from_iter(vec![
                "ccc".to_string(),
                "fff".to_string(),
                "iii".to_string(),
            ]),
        );
        expected.insert(
            "iii".to_string(),
            VecDeque::from_iter(vec!["out".to_string()]),
        );

        expected
    }
}
