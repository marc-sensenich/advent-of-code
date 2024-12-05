use advent_of_code::{read_file_to_string, read_lines};
use log::{debug, log_enabled, Level};
use std::path::Path;

pub fn part_one(input_path: &Path) -> i32 {
    let mut xmas_count: i32 = 0;
    let mut puzzle: Vec<Vec<&str>> = vec![];

    let input: String = match read_file_to_string(input_path) {
        Ok(result) => result,
        Err(_) => String::from(""),
    };

    for line in input.lines() {
        puzzle.push(line.split("").collect::<Vec<&str>>());
    }

    for (x, line) in puzzle.iter().enumerate() {
        for (y, character) in line.iter().enumerate() {
            if character == &"X" {
                let mut east_vec: Vec<&str> = vec![];
                let mut west_vec: Vec<&str> = vec![];
                let mut north_vec: Vec<&str> = vec![];
                let mut north_east_vec: Vec<&str> = vec![];
                let mut north_west_vec: Vec<&str> = vec![];
                let mut south_vec: Vec<&str> = vec![];
                let mut south_east_vec: Vec<&str> = vec![];
                let mut south_west_vec: Vec<&str> = vec![];

                for z in 1..=3 {
                    if y <= (line.len() - 3) {
                        east_vec.push(line.get(y + z).unwrap_or(&"?"));
                    }

                    if y >= 3 {
                        west_vec.push(line.get(y - z).unwrap_or(&"?"));
                    }

                    if x >= 3 {
                        north_vec.push(
                            puzzle
                                .get(x - z)
                                .unwrap_or(&vec![""])
                                .get(y)
                                .unwrap_or(&"?"),
                        );
                        if y <= (line.len() - 3) {
                            north_east_vec.push(
                                puzzle
                                    .get(x - z)
                                    .unwrap_or(&vec![""])
                                    .get(y + z)
                                    .unwrap_or(&"?"),
                            );
                        }

                        if y >= 3 {
                            north_west_vec.push(
                                puzzle
                                    .get(x - z)
                                    .unwrap_or(&vec![""])
                                    .get(y - z)
                                    .unwrap_or(&"?"),
                            );
                        }
                    }

                    if x <= (puzzle.len() - 3) {
                        south_vec.push(
                            puzzle
                                .get(x + z)
                                .unwrap_or(&vec![""])
                                .get(y)
                                .unwrap_or(&"?"),
                        );
                        if y <= (line.len() - 3) {
                            south_east_vec.push(
                                puzzle
                                    .get(x + z)
                                    .unwrap_or(&vec![""])
                                    .get(y + z)
                                    .unwrap_or(&"?"),
                            );
                        }
                        if y >= 3 {
                            south_west_vec.push(
                                puzzle
                                    .get(x + z)
                                    .unwrap_or(&vec![""])
                                    .get(y - z)
                                    .unwrap_or(&"?"),
                            );
                        }
                    }
                }

                for vec_to_check in vec![
                    north_vec,
                    north_east_vec,
                    north_west_vec,
                    east_vec,
                    west_vec,
                    south_east_vec,
                    south_vec,
                    south_west_vec,
                ] {
                    if vec_to_check.join("") == "MAS" {
                        xmas_count += 1;
                    }
                }
            }
        }
    }

    xmas_count
}

pub fn part_two(input_path: &Path) -> i32 {
    let mut xmas_count: i32 = 0;
    let mut puzzle: Vec<Vec<&str>> = vec![];

    let input: String = match read_file_to_string(input_path) {
        Ok(result) => result,
        Err(_) => String::from(""),
    };

    for line in input.lines() {
        puzzle.push(line.split("").collect::<Vec<&str>>());
    }

    for (x, line) in puzzle.iter().enumerate() {
        for (y, character) in line.iter().enumerate() {
            if character == &"A" {
                let mut north_east_char: &str = "";
                let mut north_west_char: &str = "";
                let mut south_east_char: &str = "";
                let mut south_west_char: &str = "";

                if x >= 1 {
                    if y <= (line.len() - 1) {
                        north_east_char = puzzle
                            .get(x - 1)
                            .unwrap_or(&vec![""])
                            .get(y + 1)
                            .unwrap_or(&"?");
                    }

                    if y >= 1 {
                        north_west_char = puzzle
                            .get(x - 1)
                            .unwrap_or(&vec![""])
                            .get(y - 1)
                            .unwrap_or(&"?");
                    }
                }

                if x <= (puzzle.len() - 1) {
                    if y <= (line.len() - 1) {
                        south_east_char = puzzle
                            .get(x + 1)
                            .unwrap_or(&vec![""])
                            .get(y + 1)
                            .unwrap_or(&"?");
                    }
                    if y >= 1 {
                        south_west_char = puzzle
                            .get(x + 1)
                            .unwrap_or(&vec![""])
                            .get(y - 1)
                            .unwrap_or(&"?");
                    }
                }

                let cross_left_line = format!("{}A{}", north_west_char, south_east_char);
                let cross_right_line = format!("{}A{}", north_east_char, south_west_char);

                if (cross_left_line == "SAM" || cross_left_line == "MAS")
                    && (cross_right_line == "SAM" || cross_right_line == "MAS")
                {
                    xmas_count += 1;
                }
            }
        }
    }

    xmas_count
}
