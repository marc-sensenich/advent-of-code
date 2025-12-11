use advent_of_code::read_file_to_string;
use std::path::Path;

fn solve_part_one(input: &mut Vec<char>, start: usize, offset: usize) -> u64 {
    match input.get(start) {
        Some(c) => match c {
            '.' => {
                input[start] = '|';
                solve_part_one(input, start + offset, offset)
            }
            'S' | '|' => solve_part_one(input, start + offset, offset),
            '^' => {
                input[start] = 'v';
                input[start - 1] = '|';
                input[start + 1] = '|';

                1 + solve_part_one(input, start - 1, offset)
                    + solve_part_one(input, start + 1, offset)
            }
            _ => 0,
        },
        None => 0,
    }
}

fn solve_part_two(input: &mut Vec<char>, start: usize, offset: usize) -> u64 {
    0
    // Too slow
    // match input.get(start) {
    //     Some(c) => {
    //         match c {
    //             '.' => {
    //                 solve_part_two(input, start + offset, offset)
    //             }
    //             'S' => {
    //                 solve_part_two(input, start + offset, offset)
    //             }
    //             '^' => {
    //                 solve_part_two(input, start - 1, offset) + solve_part_two(input, start + 1, offset)
    //             }
    //             _ => {
    //                 0
    //             }
    //         }
    //     }
    //     None => {
    //         1
    //     }
    // }
}

fn parse_input(input: String) -> (Vec<char>, usize, usize) {
    let offset: usize = match input.lines().last() {
        Some(l) => l.len(),
        None => 0,
    };
    let result: Vec<char> = input
        .lines()
        .map(|l| l.chars())
        .flatten()
        .collect::<Vec<char>>();
    let start: usize = result.iter().position(|c| *c == 'S').unwrap_or_else(|| 0);

    (result, start, offset)
}

pub fn part_one(input_path: &Path) -> u64 {
    match read_file_to_string(input_path) {
        Ok(input) => {
            let (mut input_vec, start, offset) = parse_input(input);
            solve_part_one(&mut input_vec, start, offset)
        }
        Err(_) => 0,
    }
}

pub fn part_two(input_path: &Path) -> u64 {
    match read_file_to_string(input_path) {
        Ok(input) => {
            let (mut input_vec, start, offset) = parse_input(input);
            solve_part_two(&mut input_vec, start, offset)
        }
        Err(_) => 0,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_AS_STRING: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    fn example_as_vec() -> Vec<char> {
        vec![
            '.', '.', '.', '.', '.', '.', '.', 'S', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            '.', '.', '.', '^', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '^', '.',
            '^', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '^', '.', '^', '.', '^', '.', '.',
            '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            '.', '.', '.', '.', '.', '^', '.', '^', '.', '.', '.', '^', '.', '.', '.', '.', '.',
            '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            '^', '.', '^', '.', '.', '.', '^', '.', '^', '.', '.', '.', '.', '.', '.', '.', '.',
            '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '^', '.', '.', '.', '^',
            '.', '.', '.', '.', '.', '^', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            '.', '.', '.', '.', '.', '.', '.', '^', '.', '^', '.', '^', '.', '^', '.', '^', '.',
            '.', '.', '^', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            '.', '.',
        ]
    }

    fn test_solve_part_one(input: &mut Vec<char>, start: usize, offset: usize, expected: u64) {
        let result = solve_part_one(input, start, offset);

        assert_eq!(result, expected,)
    }

    fn test_solve_part_two(input: &mut Vec<char>, start: usize, offset: usize, expected: u64) {
        let result = solve_part_two(input, start, offset);

        assert_eq!(result, expected,)
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input(EXAMPLE_AS_STRING.to_string()),
            (example_as_vec(), 7, 15),
        )
    }

    #[test]
    fn test_solve_part_one_simple() {
        test_solve_part_one(
            &mut vec![
                '.', '.', '.', '.', '.', '.', '.', 'S', '.', '.', '.', '.', '.', '.', '.', '.',
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                '.', '.', '.', '.', '.', '^', '.', '.', '.', '.', '.', '.', '.',
            ],
            7,
            15,
            1,
        )
    }

    #[test]
    fn test_solve_part_one_iter() {
        let (mut input, start, offset) = parse_input(
            ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
..............."
                .to_string(),
        );

        test_solve_part_one(&mut input, start, offset, 6)
    }

    #[test]
    fn test_solve_part_one_example() {
        let (mut input, start, offset) = parse_input(EXAMPLE_AS_STRING.to_string());
        test_solve_part_one(&mut input, start, offset, 21)
    }

    #[test]
    fn test_solve_part_two_example() {
        let (mut input, start, offset) = parse_input(EXAMPLE_AS_STRING.to_string());
        test_solve_part_two(&mut input, start, offset, 40)
    }
}
