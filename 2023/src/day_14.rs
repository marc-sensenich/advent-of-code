use advent_of_code::{read_lines, read_file_to_string};
use log::{debug, log_enabled, Level};
use std::path::Path;

fn tilt_rock_column (rocks: Vec<char>) -> Vec<char> {
    let mut tilted: Vec<char> = rocks.clone();
    let mut spaces_to_shift: usize = 0;

    for (idx, rock) in rocks.iter().copied().enumerate() {
        match rock {
            '.' => {
                spaces_to_shift += 1;
            },
            'O' => {
                if spaces_to_shift != 0 {
                    tilted.remove(idx);
                    tilted.insert(idx - spaces_to_shift, 'O');
                }
            }
            '#' => {
                spaces_to_shift = 0;
            }
            _ => {},
        }
    }

    tilted
}

pub fn part_one(input_path: &Path) -> u64 {
    let mut rock_columns: Vec<Vec<char>> = vec![];
    let mut tilted_columns: Vec<Vec<char>> = vec![];

    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(result) = line {
                for (idx, rock) in result.chars().enumerate() {
                    match rock_columns.get_mut(idx) {
                        Some(column) => {
                            column.push(rock);
                        }
                        None => {
                            rock_columns.push(vec![rock]);
                        }
                    }
                }

                debug!("{}", result);
            }
        }
    }

    let mut sum: u64 = 0;

    for column in rock_columns {
        let column_length = column.len();
        for (idx, rock) in tilt_rock_column(column).iter().copied().enumerate() {
            if rock == 'O' {
                sum += (column_length - idx) as u64;
            }
        }

    }

    sum
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
